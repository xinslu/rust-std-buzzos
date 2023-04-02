#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

pub mod syscalls;

pub mod testing {
    use crate::interrupts;

    use super::syscalls::{syscall1, syscall0, Sysno};
    pub unsafe fn test_syscall() {
        syscall1(Sysno::Sbrk, 10);
        syscall0(Sysno::Read);
        syscall0(Sysno::Write);
    }
}
