use core::arch::asm;

use crate::interrupts;

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
        1 => interrupts::handlers::read(),
        2 => interrupts::handlers::write(),
        _ => () 
    };
    ret
}

#[inline]
pub unsafe fn syscall1(n: Sysno, arg1: usize) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    match n as u32 {
        // 0x20 => asm!("int 0x20", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
        0 => interrupts::handlers::sbrk(arg1),
        // 1 => interrupts::handlers::read(),
        // 2 => interrupts::handlers::write(),
        _ => (0 as *mut u8) 
    };

    ret
}
