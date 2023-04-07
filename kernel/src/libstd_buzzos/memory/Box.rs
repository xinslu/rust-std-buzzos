use crate::{
    libstd_buzzos::syscalls::{syscall1, Sysno},
    println,
};
use core::{arch::asm, mem::size_of};

pub struct Box<T: ?Sized>(*mut T);

impl<T> Box<T> {
    /// Allocates memory on the heap and then places `x` into it.
    ///
    /// This doesn't actually allocate if `T` is zero-sized.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = Box::new(5);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn new(x: T) -> *const T {
        let num_bytes = size_of::<T>();
        if num_bytes <= 0 {
            return 0 as *const T;
        }
        let mem_break: *mut u8;
        mem_break = unsafe { syscall1(Sysno::Sbrk, num_bytes) as *mut u8 };
        let ptr: *const T = &x;
        unsafe {
            asm!("mov [{0}], {1}", in(reg) mem_break, in(reg) ptr, options(nomem, nostack, preserves_flags));
        }
        println!("Heap Memory: {:#?}", mem_break);
        ptr
    }

    // /// Constructs a new `Box` with uninitialized contents, with the memory
    // /// being filled with `0` bytes.
    // ///
    // /// See [`MaybeUninit::zeroed`][zeroed] for examples of correct and incorrect usage
    // /// of this method.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// #![feature(new_uninit)]
    // ///
    // /// let zero = Box::<u32>::new_zeroed();
    #[inline(always)]
    #[must_use]
    pub fn new_zeroed() -> *mut u8 {
        let num_bytes = size_of::<T>();
        let mem_break: *mut u8;
        mem_break = unsafe { syscall1(Sysno::Sbrk, num_bytes) as *mut u8 };
        mem_break
    }

    // /// Allocates memory on the heap then places `x` into it,
    // /// returning an error if the allocation fails
    // ///
    // /// This doesn't actually allocate if `T` is zero-sized.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// #![feature(allocator_api)]
    // ///
    // /// let five = Box::try_new(5)?;
    // /// # Ok::<(), std::alloc::AllocError>(())
    // /// ```
    // #[inline]
    // pub fn try_new(x: T) -> Result<T, E> {
    //     Self::try_new_in(x, Global)
    // }
}
