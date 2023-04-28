use alloc::string::String;
use spin::Mutex;

use super::{
    defs::process::{Context, Process, ProcessState, TrapFrame},
    scheduler::PROCESS_LIST,
};
use crate::{
    interrupts::defs::InterruptStackFrame,
    memory::{
        defs::{
            Page, KERNEL_BASE, KERNEL_DATA_SEG_ENTRY, PAGE_SIZE, PTE_U, PTE_W,
            TASK_SWITCH_SEG_ENTRY, USER_CODE_SEG_ENTRY, USER_DATA_SEG_ENTRY,
        },
        gdt::TSS,
        mem::{memmove, memset},
        vm::{allocate_page, map_pages, setup_kernel_page_tables},
    },
    x86::{
        defs::PrivilegeLevel,
        helpers::{load_cr3, ltr},
    },
    V2P,
};

impl Process {
    pub fn new(pid: usize) -> Self {
        Process {
            state: ProcessState::EMBRYO,
            mem_size: Default::default(),
            current_working_directory: String::from("/"),
            name: String::from(""),
            context: None,
            trapframe: None,
            kernel_stack: None,
            pgdir: None,
            pid,
        }
    }
}

extern "C" {
    static _binary_init_start: usize;
    static _binary_init_size: usize;

    pub fn trap_return();
    pub fn trap_enter(frame: InterruptStackFrame);
}

static mut NEXT_PID: Mutex<usize> = Mutex::new(0);

/// Add a process to the scheduler queue list.
pub unsafe fn queue_process(process: Process) {
    PROCESS_LIST.lock().push(process);
}

/// Spawn a process block. Notice the process block has no meaning until it is queued to be run
/// by the Scheduler. At this point, it is generally prepared to run in the user-space, but no
/// specific are provided.
pub unsafe fn spawn_process() -> Result<Process, &'static str> {
    let pid = *NEXT_PID.lock();
    let mut process = Process::new(pid);
    let trapframe_size = core::mem::size_of::<TrapFrame>() as isize;
    let context_size = core::mem::size_of::<Context>() as isize;
    let kernel_page = allocate_page()?.address as *mut usize;
    let mut esp = kernel_page.offset(PAGE_SIZE as isize / 4);

    process.kernel_stack = Some(kernel_page);
    process.mem_size = PAGE_SIZE;

    // Setup Trapframe Layout
    esp = esp.offset(-trapframe_size / 4);
    memset(esp as usize, 0, trapframe_size as usize);
    process.trapframe = Some(esp as *mut TrapFrame);

    // Setup Context Layout
    esp = esp.offset(-context_size / 4);
    memset(esp as usize, 0, context_size as usize);
    process.context = Some(esp as *mut Context);

    // Create Trap Return
    (*process.context.unwrap()).eip = trap_return as *const () as usize;

    *NEXT_PID.lock() += 1;
    Ok(process)
}

/// Switch to User Virtual Memory, departing from Kernel Virtual Memory. Notice this does
/// not yet change the DPL or CPL, as this is done at a later step. Since Kernel memory is
/// entirely mapped for every process, the process can continue executing after the memory is
/// switched.
pub unsafe fn switch_user_virtual_memory(process: &Process) {
    let page_dir = process
        .pgdir
        .expect("[FATAL] Process has no page directory") as usize;

    let mut tss = TSS.lock();
    (*tss).esp0 = process.kernel_stack.unwrap().offset(PAGE_SIZE as isize) as u32;
    (*tss).ss0 = (KERNEL_DATA_SEG_ENTRY << 3) as u16;

    ltr(TASK_SWITCH_SEG_ENTRY << 3);
    load_cr3(V2P!(page_dir));
}

/// Migrate from Kernel Virtual Memory to User Virtual Memory. Notice that
pub unsafe fn setup_user_virtual_memory(page_dir: Page, address: *const usize, size: usize) {
    if size >= PAGE_SIZE {
        panic!("[FATAL] User Virtual Memory is bigger than one page");
    }

    let memory_page = allocate_page().expect("[FATAL] Failed to Allocate");
    let virtual_address = 0;
    let page_size = PAGE_SIZE;
    let phys_address = V2P!(memory_page.address as usize);
    let flags = PTE_W | PTE_U;

    // Prepare all pages required by the process and copy over the executable binary and data
    memset(memory_page.address as usize, 0, PAGE_SIZE);
    map_pages(page_dir, virtual_address, page_size, phys_address, flags);
    memmove(address as usize, memory_page.address as usize, size);
}

/// Spawns the first process to run in the user space, the init process. Subsequent children inherit
/// many attributes of the init process, such as trapframe,
pub unsafe fn spawn_init_process() -> Result<(), &'static str> {
    let mut process = spawn_process()?;
    let kernel_pgdir = setup_kernel_page_tables()?;
    let user_code_selector = (USER_CODE_SEG_ENTRY << 3) as u16 | PrivilegeLevel::Ring3 as u16;
    let user_data_selector = (USER_DATA_SEG_ENTRY << 3) as u16 | PrivilegeLevel::Ring3 as u16;

    process.pgdir = Some(kernel_pgdir.address as *mut usize);

    setup_user_virtual_memory(
        kernel_pgdir,
        &_binary_init_start as *const usize,
        &_binary_init_size as *const usize as usize,
    );

    // Setup Trapframe
    (*process.trapframe.unwrap()).esp = PAGE_SIZE;
    (*process.trapframe.unwrap()).eip = 0;
    (*process.trapframe.unwrap()).cs = user_code_selector;
    (*process.trapframe.unwrap()).ds = user_data_selector;
    (*process.trapframe.unwrap()).es = user_data_selector;
    (*process.trapframe.unwrap()).ss = user_data_selector;
    (*process.trapframe.unwrap()).eflags = 0x0;

    // Setup Misc
    process.name = String::from("init");
    process.state = ProcessState::READY;

    queue_process(process);

    Ok(())
}
