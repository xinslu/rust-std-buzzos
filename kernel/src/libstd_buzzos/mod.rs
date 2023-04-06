#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

pub mod syscalls;
pub mod memory;
pub mod collections;

pub mod testing {
    use core::arch::asm;
    use crate::{println};
    use super::syscalls::{syscall2, Sysno};
    use crate::libstd_buzzos::memory::Box::Box;
    use crate::libstd_buzzos::collections::Vec::Vec;

    pub unsafe fn test_syscall() {
        let text: &str = "Hello";
        syscall2(Sysno::Write, text.as_ptr() as usize, text.len());

        let b = Box::<u32>::new_zeroed();
        println!("{:#?}", *b);

        let c = Box::new(1);
        println!("Box ptr: {:#?}", c);

        // Tests if alloc'd Box ptr is actually on heap (currently works)
        // unsafe{
        //     let mut ptr: u32;
        //     asm!("mov {0}, [{1}]", out(reg) ptr, in(reg) c, options(nomem, nostack, preserves_flags));
        //     println!("Heaped ptr: {:#x?}", ptr);
        // }
        
        // Vector works as long as Drop is not implemented
        let vector: Vec<u32> = Vec::with_capacity(10);
        println!("Vec ptr: {:#?}, Cap: {:#?}", vector.ptr(), vector.cap());
    }
}
