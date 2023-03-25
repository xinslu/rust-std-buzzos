#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

pub mod syscalls;

pub mod testing {
    use core::arch::asm;
    use crate::syscalls;
    pub unsafe fn test_syscall() {
        let res: usize = syscalls::syscall1(syscalls::Sysno::Sbrk, 10);
        let mut output: usize = 0;
        asm!(
            "mov {}, edx",
            out(reg) output,
        );
        panic!("Output: {:#?}", output); 
    }
}
