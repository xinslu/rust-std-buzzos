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
    interrupts::defs::system_call::*,
    println,
    scheduler::{defs::process::TrapFrame, scheduler::SCHEDULER},
};

/// If a call to an undefined System Call happens, panic and exit.
/// TODO: Change this to an exit of the process instead of killing the system.
fn panic_undefined_syscall() {
    panic!("[FATAL] Undefined System Call");
}

lazy_static! {
    /// Add your own System Calls here. Notice parameters use the System V ABI calling convention,
    /// so edi, esi, edx, and ecx registers are used to pass the first four parameters to parameters.
    /// Functions are passed as addresses in order to avoid Rust parameter validation.
    static ref SYSTEM_CALLS: [usize; NUM_SYS_CALLS] = {
        let panic_handler_address = panic_undefined_syscall as *const () as usize;
        let mut sys_calls = [panic_handler_address; NUM_SYS_CALLS];
        print_trapframe();

        sys_calls[SBRK] = sbrk as *const () as usize;
        sys_calls[WRITE] = write as *const () as usize;
        sys_calls[READ] = read as *const () as usize;

        sys_calls
    };
}

/// Every System Call passes through this handler. The trapframe is passed to facilitate loading
/// the ABI registers and getting the system call number in eax.
pub fn handle_system_call(trapframe: &TrapFrame) {
    let system_call_number = trapframe.eax;

    if system_call_number > NUM_SYS_CALLS - 1 {
        panic_undefined_syscall();
    }

    let system_call_fn = SYSTEM_CALLS[system_call_number];
    let mut res: usize = 0;
    unsafe {
        asm!("
        pusha

        mov eax, {trapframe}

        mov edi, [eax]
        mov esi, [eax + 4]
        mov ecx, [eax + 24]
        mov edx, [eax + 20]

        mov eax, ebx
        call eax

        popa
        ",
            trapframe = in(reg) trapframe as *const TrapFrame as usize,
            in("ebx") system_call_fn
        );
    }
}

pub fn print_trapframe() {
    let trapframe = unsafe { SCHEDULER.lock().get_trapframe().unwrap() };
    println!("{:#?}", unsafe { (*trapframe).clone() });
}

pub fn sbrk() {
    println!("[KERNEL] SBRK called");
    let trapframe = unsafe { *SCHEDULER.lock().get_trapframe().unwrap().clone() };
    let mut res: usize = 0;
    let mut req_size: usize = trapframe.ecx;
    let mut addr: *mut u8;
    // unsafe {
    //     asm!(
    //         "mov {}, ecx",
    //         out(reg) req_size,
    //     );
    // };

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

pub unsafe fn write() {
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
        let char = *letter.offset(i) as char;
        text.push(char);
        i += 1;
    }

    write = text.as_str();

    CONSOLE.lock().write_fmt(format_args!("{}\n", write)).unwrap();

    unsafe {
        asm!("mov eax, 1");
    };
}