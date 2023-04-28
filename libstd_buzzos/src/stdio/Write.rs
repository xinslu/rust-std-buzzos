use core::arch::asm;
use crate::syscalls::{syscall2, Sysno};

pub trait Write {
    fn write(&mut self, buf: &str) -> Result<(), ()>{
        let mut res: usize = 0;
        unsafe {
            syscall2(Sysno::Write, buf.as_ptr() as usize, buf.len());
            asm!(
                "mov {}, eax",
                out(reg) res,
            ); 
        }
        if res == 1 {
            Ok(())
        } else {
            Err(())
        }
    }
}