#![no_std]
#![no_main]
#![feature(mixed_integer_ops)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#[macro_use]

pub mod devices;
pub mod interrupts;
pub mod memory;
pub mod misc;
pub mod structures;
pub mod threading;
pub mod x86;

use core::panic::PanicInfo;
use std_buzzos;

extern crate alloc;

use core::arch::asm;

// Interface definition of panic in Rust. Core represents the core library

// Uses C calling convention instead of Rust. no_mangle removes name mangling when compiled.
// _start is the default entry point for most systems. Function is diverging as the Kernel should
// never return
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Initialize debugging method (VGA or Console)
    devices::debug::debug_init();
    misc::logo::print_logo();

    // Setup Segmentation and Virtual Memory
    memory::vm::setup_vm();
    memory::gdt::setup_gdt();
    memory::heap::setup_heap();
    // Setup Interrupts
    interrupts::idt::setup_idt();

    std_buzzos::testing::test_syscall();

    
    loop {}
}

// Once the Kernel panics, enter an infinite loop
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!("{}", _info);
    loop {}
}

#[alloc_error_handler]
fn alloc_panic(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
