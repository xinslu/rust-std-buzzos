use core::arch::asm;
use lazy_static::lazy_static;

use alloc::string::String;
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

use crate::memory::heap::HEAP_ALLOCATOR;

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
    unsafe { SCHEDULER.lock().resume() };
}

pub fn exit() {
    unsafe {
        let mut scheduler = SCHEDULER.lock();
        scheduler.current_process.as_mut().unwrap().state = ProcessState::KILLED;
        scheduler.resume();
    }
}

pub fn sbrk() {
    println!("[KERNEL] SBRK called");
    let mut res: usize = 0;
    let mut req_size: usize = 0;
    let mut addr: *mut u8;
    unsafe {
        asm!(
            "mov {}, ecx",
            out(reg) req_size,
        );
    };

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
    let mut count: usize = 0;
    let mut res: usize = 0;

    unsafe {
        asm!(
            "mov {}, ecx",
            out(reg) count,
        );
    };

    if count == 0 {
        unsafe {
            asm!("mov eax, 1");
        };
    }
    unsafe {
        asm!("mov eax, 1");
    };
}

pub fn write() {
    println!("[KERNEL] WRITE called");
    let letter: *const u8;
    let mut len: usize = 0;
    unsafe {
        asm!(
            "mov {}, ecx",
            out(reg) letter,
        );

        asm!(
            "mov {}, edx",
            out(reg) len,
        );
    }

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
    println!("{:#?}", write);
    unsafe {
        asm!("mov eax, 1");
    };
}
