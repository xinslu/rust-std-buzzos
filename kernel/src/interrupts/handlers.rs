use core::{alloc::{GlobalAlloc, Layout}};
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
    println!("Page Fault at virtual address {:#x?}", addr);
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
    panic!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", frame);
}

pub fn sbrk(frame: InterruptStackFrame, _err: u16) {
    let mut res: usize = 0;
    
    unsafe {
        asm!(
            "mov {}, edx",
            out(reg) res,
        );
    };

    let layout: Layout;
    println!("{}", res);
    match Layout::from_size_align(res, 4) {
        Ok(x) => layout = x, 
        Err(y) => panic!("Layou Error: {}", y)
    };

    let mem_break: *mut u8;
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
    }
    unsafe {
        asm!(
            "mov eax, {}",
            in(reg) mem_break,
        );
    };
    println!("TRAP: SBRK SYSCALL got {:#?} bytes starting at {:#x?} \n", res, mem_break);

}
