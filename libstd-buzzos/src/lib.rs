#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

pub mod syscalls;

pub mod testing {
    use crate::syscalls;
    pub unsafe fn test_syscall() {
        let res: usize = syscalls::syscall0(syscalls::Sysno::Sbrk);
    }
}
