#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]

pub mod syscalls;

pub mod testing {
    use crate::interrupts;
    pub unsafe fn test_syscall() {
        let addr: *mut u8 = interrupts::handlers::sbrk(10);
        interrupts::handlers::read();
        interrupts::handlers::write();

    }
}
