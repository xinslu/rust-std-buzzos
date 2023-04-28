#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    main(0, core::ptr::null());
}

#[lang = "start"]
fn lang_start<T>(main: fn() -> T, _: isize, _: *const *const u8) -> isize {
    main();
    0
}