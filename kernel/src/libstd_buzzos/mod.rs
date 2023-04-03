#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

pub mod syscalls;

pub mod testing {
    use crate::interrupts;

    use super::syscalls::{syscall2, syscall3, Sysno};
    pub unsafe fn test_syscall() {
        let mut mem_break = 0;
        syscall2(Sysno::Sbrk, 10, mem_break);
        syscall3(Sysno::Read, 0, 0, 1);
        syscall3(Sysno::Write, 0, 0, 1);
    }
}
