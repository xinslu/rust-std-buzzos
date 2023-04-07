use core::ptr::{NonNull, null_mut};
use core::mem::size_of;
use core::alloc::{Layout, GlobalAlloc};
use crate::{libstd_buzzos::syscalls::{syscall1, Sysno}};
use crate::memory::heap::HEAP_ALLOCATOR;

pub struct RawVec<T> {
    pub ptr: NonNull<T>,
    pub cap: usize,
}

impl<T> RawVec<T> {
    pub fn new() -> Self {
        assert!(size_of::<T>() != 0, "Size must be greater than 0");
        RawVec {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        assert!(size_of::<T>() != 0, "Size must be greater than 0");
        let mut raw_vec = RawVec {
            ptr: NonNull::dangling(),
            cap: capacity, //allocate capacity on heap first
        };
        raw_vec.grow(true);
        raw_vec
    }

    pub fn grow(&mut self, is_init: bool) {
        // This can't overflow because we ensure self.cap <= isize::MAX.
        let new_cap = if self.cap == 0 {
            1 
        } else if is_init {
            self.cap
        } else { 
            2* self.cap
        };

        let new_layout = Layout::array::<T>(new_cap).unwrap();
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");
        
        // Layout::array checks that the number of bytes is <= usize::MAX,
        // but this is redundant since old_layout.size() <= isize::MAX,
        // so the `unwrap` should never fail.
        let new_ptr: *mut u8 = if self.cap == 0 {
            unsafe{syscall1(Sysno::Sbrk, new_cap) as *mut u8} 
        } else {
            unsafe{syscall1(Sysno::Sbrk, self.cap) as *mut u8} 
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        // assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => NonNull::new(null_mut()).unwrap(),
        };

        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                HEAP_ALLOCATOR.dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}
