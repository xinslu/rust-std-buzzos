#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(allocator_internals)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#![feature(ptr_internals)]
#![feature(decl_macro)]
#![feature(coerce_unsized)]
#![feature(const_trait_impl)]
#![feature(negative_impls)]
#![feature(unsize)]
#![feature(allow_internal_unstable)]
#![feature(rustc_attrs)]
#![feature(never_type)]
#![feature(lang_items)]
#![default_lib_allocator]
#[macro_use]

pub mod collections;
pub mod memory;
pub mod syscalls;
pub mod types;
extern crate alloc;
use core::panic::PanicInfo;

pub mod testing {
    // use super::syscalls::{syscall2, Sysno};
    // use crate::collections::Vec::Vec;
    // use crate::collections::VecDeque::VecDeque;
    // use crate::memory::Box::Box;
    // use crate::memory::Clone::Clone;
    // use crate::types::String::String;

    pub unsafe fn test_syscall() {
        // let text: &str = "Hello";
        // // syscall2(Sysno::Write, text.as_ptr() as usize, text.len());

        // let b = Box::<u32>::new_zeroed();
        // // println!("{:#?}", *b);

        // let c = Box::new(1);
        // // println!("Box ptr: {:#?}", c);

        // // Tests if alloc'd Box ptr is actually on heap (currently works)
        // // unsafe{
        // //     let mut ptr: u32;
        // //     asm!("mov {0}, [{1}]", out(reg) ptr, in(reg) c, options(nomem, nostack, preserves_flags));
        // //     println!("Heaped ptr: {:#x?}", ptr);
        // // }
        // clone_tests();
        // vector_tests();
        // deque_tests();
        // // string_tests();
    }

    // Tests basic vector initialization with push/pop/clear
    pub unsafe fn vector_tests() {
        // let mut vector: Vec<u32> = Vec::with_capacity(10);
        // println!("Vec ptr: {:#?}, Cap: {:#?}", vector.ptr(), vector.cap());
        // vector.push(1);
        // vector.push(80);
        // println!("Vector element 0: {:#?}", *vector.ptr());
        // println!("Vector element 1: {:#?}", *vector.ptr().offset(1));
        // let val: Option<u32> = vector.pop();
        // println!("Vector element 0: {:#?}", *vector.ptr());
        // println!("Popped val: {:#?}", val.unwrap());
        // println!("Vector Length: {:#?}", vector.len());
        // vector.push(10);
        // println!("Vector element 0: {:#?}", *vector.ptr());
        // println!("Vector element 1: {:#?}", *vector.ptr().offset(1));
        // println!("Vector Length: {:#?}", vector.len());
        // vector.clear();
        // println!("Vector Length: {:#?}", vector.len());
        // println!("Vector element 0: {:#?}", *vector.ptr());
        // println!("Is empty? {:#?}", vector.is_empty());
    }

    // Tests basic string initialization
    pub unsafe fn string_tests() {
        // let mut string: String = String::from("Hello World!");
        // println!("Len: {:#?}", string.len());
        // println!("Char 1: {:#?}", string.get_char_at(1));
        // string.push('Y');
        // assert!(string.get_char_at(12) == 'Y');
        // assert!(string.pop().unwrap() == 'Y');
        // assert!(string.len() == 12);
    }

    // Tests basic Deque initialization
    pub unsafe fn deque_tests() {
        // let mut vd: VecDeque<u32> = VecDeque::new();
        // vd.push_front(1);
        // println!("Head: {:#?}", (*vd.head).value);
        // println!("Tail: {:#?}", (*vd.tail).value);
        // vd.push_back(80);
        // println!("Head: {:#?}", (*vd.head).value);
        // println!("Tail: {:#?}", (*vd.tail).value);
        // vd.pop_front();
        // println!("Head: {:#?}", (*vd.head).value);
        // println!("Tail: {:#?}", (*vd.tail).value);
    }

    pub unsafe fn clone_tests() {
        // let string: String = String::from("Hello World!");
        // let string2: String = string.clone();
        // println!("{}", string2);
    }
}


// Once the Kernel panics, enter an infinite loop
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_panic(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}