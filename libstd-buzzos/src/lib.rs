#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

use core::arch::asm;

pub enum Sysno {
    Sbrk = 0,
}

#[inline]
pub unsafe fn syscall0(n: Sysno) -> usize {
    let mut ret: usize = 0;
    asm!(
        "int 0x0",
        inlateout("eax") (n as usize) => ret,
        options(nostack, preserves_flags)
    );
    ret
}

#[inline]
pub unsafe fn syscall1(n: Sysno, arg1: usize) -> usize {
    let mut ret: usize = 0;
    asm!(
        "int 0x0",
        inlateout("eax") (n as usize) => ret,
        in("ebx") arg1,
        options(nostack, preserves_flags)
    );
    ret
}

pub unsafe fn main() {
    let res: usize = syscall0(Sysno::Sbrk);
}
