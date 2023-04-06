use core::ptr::{NonNull, null_mut};
use core::mem::size_of;
use core::alloc::{Layout, GlobalAlloc};

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
            cap: capacity/2, //allocate capacity on heap first
        };
        raw_vec.grow();
        raw_vec
    }

    pub fn grow(&mut self) {
        // This can't overflow because we ensure self.cap <= isize::MAX.
        let new_cap = if self.cap == 0 { 1 } else { 2 * self.cap };
        
        // Layout::array checks that the number of bytes is <= usize::MAX,
        // but this is redundant since old_layout.size() <= isize::MAX,
        // so the `unwrap` should never fail.
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr: *mut u8 = if self.cap == 0 {
            unsafe { HEAP_ALLOCATOR.alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            // let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { HEAP_ALLOCATOR.alloc(old_layout)}
        };

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