

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
    // #[cfg(all(not(no_global_oom_handling)))]
    // #[inline(always)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // #[must_use]
    // pub fn new(x: T) -> Self {
    //     #[rustc_box]
    //     Box::new(x)
    // }

    // /// Constructs a new box with uninitialized contents.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// #![feature(new_uninit)]
    // ///
    // /// let mut five = Box::<u32>::new_uninit();
    // ///
    // /// let five = unsafe {
    // ///     // Deferred initialization:
    // ///     five.as_mut_ptr().write(5);
    // ///
    // ///     five.assume_init()
    // /// };
    // ///
    // /// assert_eq!(*five, 5)
    // /// ```
    // #[cfg(not(no_global_oom_handling))]
    // #[unstable(feature = "new_uninit", issue = "63291")]
    // #[must_use]
    // #[inline]
    // pub fn new_uninit() -> Box<mem::MaybeUninit<T>> {
    //     Self::new_uninit_in(Global)
    // }

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
    // /// let zero = unsafe { zero.assume_init() };
    // ///
    // /// assert_eq!(*zero, 0)
    // /// ```
    // ///
    // /// [zeroed]: mem::MaybeUninit::zeroed
    // #[cfg(not(no_global_oom_handling))]
    // #[inline]
    // #[unstable(feature = "new_uninit", issue = "63291")]
    // #[must_use]
    // pub fn new_zeroed() -> Box<mem::MaybeUninit<T>> {
    //     Self::new_zeroed_in(Global)
    // }

    // /// Constructs a new `Pin<Box<T>>`. If `T` does not implement [`Unpin`], then
    // /// `x` will be pinned in memory and unable to be moved.
    // ///
    // /// Constructing and pinning of the `Box` can also be done in two steps: `Box::pin(x)`
    // /// does the same as <code>[Box::into_pin]\([Box::new]\(x))</code>. Consider using
    // /// [`into_pin`](Box::into_pin) if you already have a `Box<T>`, or if you want to
    // /// construct a (pinned) `Box` in a different way than with [`Box::new`].
    // #[cfg(not(no_global_oom_handling))]
    // #[stable(feature = "pin", since = "1.33.0")]
    // #[must_use]
    // #[inline(always)]
    // pub fn pin(x: T) -> Pin<Box<T>> {
    //     (#[rustc_box]
    //     Box::new(x))
    //     .into()
    // }

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
    // #[unstable(feature = "allocator_api", issue = "32838")]
    // #[inline]
    // pub fn try_new(x: T) -> Result<Self, AllocError> {
    //     Self::try_new_in(x, Global)
    // }

    // /// Constructs a new box with uninitialized contents on the heap,
    // /// returning an error if the allocation fails
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// #![feature(allocator_api, new_uninit)]
    // ///
    // /// let mut five = Box::<u32>::try_new_uninit()?;
    // ///
    // /// let five = unsafe {
    // ///     // Deferred initialization:
    // ///     five.as_mut_ptr().write(5);
    // ///
    // ///     five.assume_init()
    // /// };
    // ///
    // /// assert_eq!(*five, 5);
    // /// # Ok::<(), std::alloc::AllocError>(())
    // /// ```
    // #[unstable(feature = "allocator_api", issue = "32838")]
    // // #[unstable(feature = "new_uninit", issue = "63291")]
    // #[inline]
    // pub fn try_new_uninit() -> Result<Box<mem::MaybeUninit<T>>, AllocError> {
    //     Box::try_new_uninit_in(Global)
    // }

    // /// Constructs a new `Box` with uninitialized contents, with the memory
    // /// being filled with `0` bytes on the heap
    // ///
    // /// See [`MaybeUninit::zeroed`][zeroed] for examples of correct and incorrect usage
    // /// of this method.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// #![feature(allocator_api, new_uninit)]
    // ///
    // /// let zero = Box::<u32>::try_new_zeroed()?;
    // /// let zero = unsafe { zero.assume_init() };
    // ///
    // /// assert_eq!(*zero, 0);
    // /// # Ok::<(), std::alloc::AllocError>(())
    // /// ```
    // ///
    // /// [zeroed]: mem::MaybeUninit::zeroed
    // #[unstable(feature = "allocator_api", issue = "32838")]
    // // #[unstable(feature = "new_uninit", issue = "63291")]
    // #[inline]
    // pub fn try_new_zeroed() -> Result<Box<mem::MaybeUninit<T>>, AllocError> {
    //     Box::try_new_zeroed_in(Global)
    // }
}