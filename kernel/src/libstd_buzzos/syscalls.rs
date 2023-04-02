use core::arch::asm;

#[derive(Copy, Clone)]
pub enum Sysno {
    Sbrk = 0,
}

const SYS_START : u32 = 0x20;


#[inline]
pub unsafe fn syscall0(n: Sysno) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);

    match address {
        0x20 => asm!("int 0x20", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x21 => asm!("int 0x21", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x22 => asm!("int 0x22", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x23 => asm!("int 0x23", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x24 => asm!("int 0x24", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x25 => asm!("int 0x25", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x26 => asm!("int 0x26", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x27 => asm!("int 0x27", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x28 => asm!("int 0x28", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x29 => asm!("int 0x29", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x2A => asm!("int 0x2A", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x2B => asm!("int 0x2B", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x2C => asm!("int 0x2C", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x2D => asm!("int 0x2D", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x2E => asm!("int 0x2E", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        0x2F => asm!("int 0x2F", inlateout("eax") (n as usize) => ret, options(nostack, preserves_flags)),
        _ => () 
    };
    ret
}

#[inline]
pub unsafe fn syscall1(n: Sysno, arg1: usize) -> usize {
    let mut ret: usize = 0;
    let address: u32 = SYS_START + (n as u32);
    asm!("int 0x20", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags));

    // match address {
    //     0x20 => asm!("int 0x20", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x21 => asm!("int 0x21", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x22 => asm!("int 0x22", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x23 => asm!("int 0x23", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x24 => asm!("int 0x24", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x25 => asm!("int 0x25", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x26 => asm!("int 0x26", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x27 => asm!("int 0x27", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x28 => asm!("int 0x28", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x29 => asm!("int 0x29", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x2A => asm!("int 0x2A", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x2B => asm!("int 0x2B", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x2C => asm!("int 0x2C", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x2D => asm!("int 0x2D", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x2E => asm!("int 0x2E", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     0x2F => asm!("int 0x2F", inlateout("eax") (n as usize) => ret, in("edx") arg1, options(nostack, preserves_flags)),
    //     _ => () 
    // };

    ret
}
