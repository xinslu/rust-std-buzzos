use core::{arch::asm, ffi::c_void};

use crate::{interrupts, println, print};

#[derive(Copy, Clone)]
pub enum Sysno {
    Sbrk = 0,
    Read = 1,
    Write = 2,
}

const SYS_START : u32 = 0x20;

#[inline]
pub unsafe fn syscall0(n: Sysno) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    match n as u32 {
        // 0x20 => asm!("int 0x20", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        // 0 => interrupts::handlers::sbrk(0),
        // 1 => interrupts::handlers::read(),
        // 2 => interrupts::handlers::write(),
        _ => 0 
    };
    ret
}

#[inline]
pub unsafe fn syscall1(n: Sysno, arg1: usize) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    asm!("mov ecx, {}", in(reg) arg1);
    match n as u32 {
        // 0x20 => asm!("int 0x20", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
        0 => return interrupts::handlers::sbrk() as usize,
        // 1 => interrupts::handlers::read(),
        // 2 => return interrupts::handlers::write(),
        _ => 0
    };

    ret
}

#[inline]
pub unsafe fn syscall2(n: Sysno, arg1: usize, arg2: usize) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    asm!("nop", in("ecx") arg1, in("edx") arg2);
    match n as u32 {
        // 0x20 => asm!("int 0x20", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
        // 0 => interrupts::handlers::sbrk(),
        // 1 => interrupts::handlers::read(),
        2 => return interrupts::handlers::write(),
        _ => 0 
    };
    // println!("{:#?}", arg2);
    ret
}

#[inline]
pub unsafe fn syscall3(n: Sysno, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    asm!("nop", in("ecx") arg1, in("edx") arg2, in("eax") arg3);
    match n as u32 {
        // 0x20 => asm!("int 0x20", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
        // 0 => interrupts::handlers::sbrk(arg1, arg2 as *mut u8),
        1 => interrupts::handlers::read(arg1, arg2 as *mut c_void, arg3),
        // 2 => interrupts::handlers::write(arg1, arg2 as *mut c_void, arg3),
        _ => 0, 
    };

    ret
}