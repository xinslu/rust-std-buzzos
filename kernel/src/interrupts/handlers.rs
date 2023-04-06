use core::{alloc::{GlobalAlloc, Layout}, ffi::c_void, fmt};
use alloc::{string::String};

use crate::{println, memory::{heap::HEAP_ALLOCATOR, defs::LinkedListAllocator}, devices::console::{CONSOLE, Console}};
use super::defs::{InterruptStackFrame, PageFaultErr};
use core::arch::asm;

pub extern "x86-interrupt" fn div_by_zero_handler(frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVISION BY ZERO\n{:#?}", frame);
}

pub extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", frame);
}

pub extern "x86-interrupt" fn page_fault(frame: InterruptStackFrame, _error_code: PageFaultErr) {
    let addr: u32;
    unsafe {
        asm!(
            "mov {}, cr2",
            out(reg) addr,
        );
    };
    println!("Page Fault at virtual address: {:#x?}", addr);
    panic!("EXCEPTION: PAGE FAULT\n{:#?}", frame);
}

pub extern "x86-interrupt" fn non_maskable(frame: InterruptStackFrame) {
    println!("EXCEPTION: NON MASKABLE INTERRUPT\n{:#?}", frame);
}

pub extern "x86-interrupt" fn overflow(frame: InterruptStackFrame) {
    println!("EXCEPTION: OVERFLOW\n{:#?}", frame);
}

pub extern "x86-interrupt" fn bound_range(frame: InterruptStackFrame) {
    println!("EXCEPTION: BOUND RANGE EXCEEDED\n{:#?}", frame);
}

pub extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, _err: u32) {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#X?}", frame);
}

pub extern "x86-interrupt" fn gen_protection_fault(frame: InterruptStackFrame, _err: u32) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}, error code: {_err:#b}", frame);
}

pub fn sbrk() -> *mut u8 {
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
        Err(y) => panic!("Layout Error: {}", y)
    };

    unsafe {
        addr = HEAP_ALLOCATOR.alloc(layout);
    };

    if addr.is_null() {
        println!("TRAP: SBRK SYSCALL got no free memory\n");
        return 0 as *mut u8;
    }
    println!("TRAP: SBRK SYSCALL got {:#?} bytes starting at {:#x?}", req_size, addr);
    return addr;
}

pub fn read(fd: usize, buf: *mut c_void, count: usize) -> usize {
    let mut res: usize = 0;
    if count == 0 {
        res = 1;
        return res;
    }
    println!("TRAP: SYSREAD");
    res
}

pub unsafe fn write() -> usize {
    let letter: *const u8;
    let mut len: usize = 0;
    asm!(
        "mov {}, ecx",
        out(reg) letter,
    );
    asm!(
        "mov {}, edx",
        out(reg) len,
    );

    if len == 0 {
        return 0;
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
    // let console: Console = todo!();
    // Console::write_string(&console, write);
    println!("{:#?}", write);
    return 1;
}