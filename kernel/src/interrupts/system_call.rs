use core::arch::asm;
use lazy_static::lazy_static;
use core::fmt::Write;

use alloc::string::String;
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

use crate::{memory::heap::HEAP_ALLOCATOR, devices::console::CONSOLE};

use crate::{
    interrupts::defs::system_call as SystemCall,
    println,
    scheduler::{
        defs::process::ProcessState,
        scheduler::{PROCESS_LIST, SCHEDULER},
    },
};

/// If a call to an undefined System Call happens, panic and exit.
/// TODO: Change this to an exit of the process instead of killing the system.
fn panic_undefined_syscall() {
    panic!("[FATAL] Undefined System Call");
}

/// Every System Call passes through this handler. The trapframe is passed to facilitate loading
/// the ABI registers and getting the system call number in eax.
pub fn handle_system_call(number: usize, arg0: usize, arg1: usize, arg2: usize, arg3: usize) {
    let system_call_fn = match number {
        SystemCall::SBRK => sbrk(),
        SystemCall::WRITE => write(),
        SystemCall::READ => read(),
        SystemCall::PRINT_TRAP_FRAME => print_trapframe(),
        SystemCall::EXIT => exit(),
        SystemCall::YIELD => _yield(),
        _ => panic_undefined_syscall(),
    };
}

pub fn print_trapframe() {
    let scheduler = unsafe { SCHEDULER.lock() };
    let current_process = scheduler.current_process.as_ref().unwrap();
    let trapframe = current_process.get_trapframe().unwrap();
    println!("{:#?}", trapframe);
}

pub fn _yield() {
    println!("[KERNEL] YEILD called");
    unsafe { SCHEDULER.lock().resume() };
}

pub fn exit() {
    println!("[KERNEL] EXIT called");
    unsafe {
        let mut scheduler = SCHEDULER.lock();
        scheduler.current_process.as_mut().unwrap().state = ProcessState::KILLED;
        scheduler.resume();
    }
}

pub fn sbrk() {
    println!("[KERNEL] SBRK called");
    let trapframe = unsafe { *SCHEDULER.lock().get_trapframe().unwrap().clone() };
    let mut res: usize = 0;
    let mut req_size: usize = trapframe.ecx;
    let mut addr: *mut u8;

    let layout: Layout;
    match Layout::from_size_align(req_size, 4) {
        Ok(x) => layout = x,
        Err(y) => {
            println!("[KERNEL] Layout Error: {}", y);
            unsafe {
                asm!("mov eax, 0");
            };
            return;
        }
    };

    unsafe {
        addr = HEAP_ALLOCATOR.alloc(layout);
    };

    if addr.is_null() {
        println!("[KERNEL] SBRK got no free memory\n");
        unsafe {
            asm!("mov eax, 0");
        };
    }
    println!(
        "[KERNEL] SBRK got {:#?} bytes starting at {:#x?}",
        req_size, addr
    );
    unsafe {
        asm!(
            "mov eax, {}",
            in(reg) addr,
        );
    };
}

pub fn read() {
    println!("[KERNEL] READ called");
    let trapframe = unsafe { *SCHEDULER.lock().get_trapframe().unwrap().clone() };
    let letter: *const u8 = trapframe.ecx as *const u8;
    let mut len: usize = trapframe.edx;
    let fd: u8 = trapframe.edi as u8;

    let mut res: usize = 0;

    // Can't read anything yet (no file descriptors)
}

pub fn write() {
    println!("[KERNEL] WRITE called");
    let trapframe = unsafe { *SCHEDULER.lock().get_trapframe().unwrap().clone() };

    let letter: *const u8 = trapframe.ecx as *const u8;
    let mut len: usize = trapframe.edx;

    // Switches between printing methods
    if len == 0 {
        unsafe {
            asm!("mov eax, 0");
        };
        return;
    }

    let mut i: isize = 0;
    let mut write: &str;
    let mut text: String = String::new();
    while i < len as isize {
        let char: char;
        unsafe {
            char = *letter.offset(i) as char;
        }
        text.push(char);
        i += 1;
    }

    write = text.as_str();

    CONSOLE.lock().write_fmt(format_args!("{}\n", write)).unwrap();

    unsafe {
        asm!("mov eax, 1");
    };
}