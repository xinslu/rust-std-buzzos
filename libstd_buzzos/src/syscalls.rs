use core::{arch::asm};

#[derive(Copy, Clone)]
pub enum Sysno {
    Sbrk = 0,
    Read = 1,
    Write = 2,
}

const SYS_START: u32 = 0x20;

#[inline]
pub unsafe fn syscall0(n: Sysno) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    asm!("int 64",
        inlateout("eax") n as usize => ret,
        options(nostack, preserves_flags));

    ret
}

#[inline]
pub unsafe fn syscall1(n: Sysno, arg1: usize) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    asm!("int 64",
        inlateout("eax") n as usize => ret,
        in("ecx") arg1,
        options(nostack, preserves_flags));
    ret
}

#[inline]
pub unsafe fn syscall2(n: Sysno, arg1: usize, arg2: usize) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    asm!("int 64",
        inlateout("eax") n as usize => ret,
        in("ecx") arg1,
        in("edx") arg2,
        options(nostack, preserves_flags));
    ret
}

#[inline]
pub unsafe fn syscall3(n: Sysno, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    asm!("int 64",
        inlateout("eax") n as usize => ret,
        in("ecx") arg1,
        in("edx") arg2,
        in("edi") arg3,
        options(nostack, preserves_flags));
    ret
}