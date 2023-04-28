use core::arch::asm;
use crate::syscalls::{syscall3, Sysno};

pub trait Read {
    fn read(&mut self, buf: &str, fd: u8) -> Result<(), ()>{
        let mut res: usize = 0;
        unsafe {
            syscall3(Sysno::Read, buf.as_ptr() as usize, buf.len(), fd);
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
