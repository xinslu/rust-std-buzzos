#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

pub mod syscalls;

pub mod testing {
    use core::arch::asm;
    use crate::{interrupts, println};
    use super::syscalls::{syscall1, syscall2, Sysno};
    pub unsafe fn test_syscall() {
        let mem_break: *mut u8;
        mem_break = syscall1(Sysno::Sbrk, 10) as *mut u8;
        println!("Addr: {:#?}", mem_break);

        let text: &str = "Hello";
        syscall2(Sysno::Write, text.as_ptr() as usize, text.len());
    }
}
