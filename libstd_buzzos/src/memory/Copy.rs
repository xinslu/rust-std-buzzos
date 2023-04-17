use crate::memory::Clone;

#[rustc_unsafe_specialization_marker]
pub trait Copy: Clone::Clone {
    // Empty.
}

/// Derive macro generating an impl of the trait `Copy`.
#[allow_internal_unstable(core_intrinsics, derive_clone_copy)]
pub macro Copy($item:item) {
    /* compiler built-in */
}

/// Implementations of `Copy` for primitive types.
///
/// Implementations that cannot be described in Rust
/// are implemented in `traits::SelectionContext::copy_clone_conditions()`
/// in `rustc_trait_selection`.
mod copy_impls {

    use super::Copy;

    macro_rules! impl_copy {
        ($($t:ty)*) => {
            $(
                impl Copy for $t {}
            )*
        }
    }

    impl_copy! {
        usize u8 u16 u32 u64 u128
        isize i8 i16 i32 i64 i128
        f32 f64
        bool char
    }

    impl Copy for ! {}

    impl<T: ?Sized> Copy for *const T {}

    impl<T: ?Sized> Copy for *mut T {}

    /// Shared references can be copied, but mutable references *cannot*!
    impl<T: ?Sized> Copy for &T {}
}
