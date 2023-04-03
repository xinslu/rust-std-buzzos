use core::{alloc::{GlobalAlloc, Layout}, ffi::c_void};
use crate::{println, memory::{heap::HEAP_ALLOCATOR, defs::LinkedListAllocator}};
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
    println!("Page Fault at virtual addreq_sizes {:#x?}", addr);
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

pub fn sbrk(req_size: usize, mut mem_break: *mut u8) -> usize {
    let mut res: usize = 0;
    let layout: Layout;
    println!("{}", req_size);
    match Layout::from_size_align(req_size, 4) {
        Ok(x) => layout = x, 
        Err(y) => panic!("Layout Error: {}", y)
    };

    unsafe {
        mem_break = HEAP_ALLOCATOR.alloc(layout);
    };

    if mem_break.is_null() {
        unsafe {
            asm!(
                "mov eax, {}",
                in(reg) -1,
            );
        };
        println!("TRAP: SBRK SYSCALL got no free memory\n");
        return res;
    }
    unsafe {
        asm!(
            "mov eax, {}",
            in(reg) mem_break,
        );
    };
    println!("TRAP: SBRK SYSCALL got {:#?} bytes starting at {:#x?}", req_size, mem_break);
    res = 1;
    res
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

pub fn write(fd: usize, buf: *const c_void, count: usize) -> usize {
    let mut res: usize = 0;
    if count == 0 {
        res = 1;
        return res;
    }
    println!("TRAP: SYSWRITE");
    res
}