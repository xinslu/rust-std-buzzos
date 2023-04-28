#![no_std]
#![feature(allocator_internals)]
#![feature(lang_items)]
#![feature(start)]
#![default_lib_allocator]

use stdbuzz::string_tests;
use stdbuzz::types::String::String;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    loop {}
}

#[lang = "start"]
fn lang_start<T>(main: fn() -> T, _: isize, _: *const *const u8, _: u8) -> isize {
    main();
    0
}

#[start]
fn main(_: isize, _: *const *const u8) -> isize {
    String::new();
    0
}

