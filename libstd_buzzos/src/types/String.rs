use alloc::borrow::ToOwned;

use crate::collections::Vec::Vec;
use crate::memory::Box::Box;

use core::fmt;
use core::ptr;
use core::slice;

#[derive(Clone)]
pub struct String {
    vec: Vec<u8>,
}

impl String {
    /// Creates a new empty `String`.
    ///
    /// Given that the `String` is empty, this will not allocate any initial
    /// buffer. While that means that this initial operation is very
    /// inexpensive, it may cause excessive allocation later when you add
    /// data. If you have an idea of how much data the `String` will hold,
    /// consider the [`with_capacity`] method to prevent excessive
    /// re-allocation.
    ///
    /// [`with_capacity`]: String::with_capacity
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = String::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> String {
        String { vec: Vec::new() }
    }

    /// Creates a new empty `String` with at least the specified capacity.
    ///
    /// `String`s have an internal buffer to hold their data. The capacity is
    /// the length of that buffer, and can be queried with the [`capacity`]
    /// method. This method creates an empty `String`, but one with an initial
    /// buffer that can hold at least `capacity` bytes. This is useful when you
    /// may be appending a bunch of data to the `String`, reducing the number of
    /// reallocations it needs to do.
    ///
    /// [`capacity`]: String::capacity
    ///
    /// If the given capacity is `0`, no allocation will occur, and this method
    /// is identical to the [`new`] method.
    ///
    /// [`new`]: String::new
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = String::with_capacity(10);
    ///
    /// // The String contains no chars, even though it has capacity for more
    /// assert_eq!(s.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// let cap = s.capacity();
    /// for _ in 0..10 {
    ///     s.push('a');
    /// }
    ///
    /// assert_eq!(s.capacity(), cap);
    ///
    /// // ...but this may make the string reallocate
    /// s.push('a');
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> String {
        String {
            vec: Vec::with_capacity(capacity),
        }
    }

    // /// Extracts a string slice containing the entire `String`.
    // ///
    // /// # Examples
    // ///
    // /// Basic usage:
    // ///
    // /// ```
    // /// let s = String::from("foo");
    // ///
    // /// assert_eq!("foo", s.as_str());
    // /// ```
    // #[inline]
    // #[must_use]
    // pub fn as_str(&self) -> &str {
    //     self
    // }

    /// Returns this `String`'s capacity, in bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = String::with_capacity(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.vec.cap()
    }

    /// Appends the given [`char`] to the end of this `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = String::from("abc");
    ///
    /// s.push('1');
    /// s.push('2');
    /// s.push('3');
    ///
    /// assert_eq!("abc123", s);
    /// ```
    #[inline]
    pub fn push(&mut self, ch: char) {
        self.vec.push(ch as u8)
    }

    /// Appends the given [`char`] to the end of this `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = String::from("abc");
    ///
    /// s.push('1');
    /// s.push('2');
    /// s.push('3');
    ///
    /// assert_eq!("abc123", s);
    /// ```
    #[inline]
    pub fn get_char_at(&self, index: usize) -> char {
        self.vec.get(index) as char
    }

    // /// Removes the last character from the string buffer and returns it.
    // ///
    // /// Returns [`None`] if this `String` is empty.
    // ///
    // /// # Examples
    // ///
    // /// Basic usage:
    // ///
    // /// ```
    // /// let mut s = String::from("foo");
    // ///
    // /// assert_eq!(s.pop(), Some('o'));
    // /// assert_eq!(s.pop(), Some('o'));
    // /// assert_eq!(s.pop(), Some('f'));
    // ///
    // /// assert_eq!(s.pop(), None);
    // /// ```
    // #[inline]
    pub fn pop(&mut self) -> Option<char> {
        Some(*self.vec.pop().as_ref().unwrap() as char)
    }

    /// Truncates this `String`, removing all contents.
    ///
    /// While this means the `String` will have a length of zero, it does not
    /// touch its capacity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut s = String::from("foo");
    ///
    /// s.clear();
    ///
    /// assert!(s.is_empty());
    /// assert_eq!(0, s.len());
    /// assert_eq!(3, s.capacity());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear()
    }

    /// Returns the length of this `String`, in bytes, not [`char`]s or
    /// graphemes. In other words, it might not be what a human considers the
    /// length of the string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let a = String::from("foo");
    /// assert_eq!(a.len(), 3);
    ///
    /// let fancy_f = String::from("ƒoo");
    /// assert_eq!(fancy_f.len(), 4);
    /// assert_eq!(fancy_f.chars().count(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns `true` if this `String` has a length of zero, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let mut v = String::new();
    /// assert!(v.is_empty());
    ///
    /// v.push('a');
    /// assert!(!v.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Converts a `&str` into a [`String`].
    ///
    /// The result is allocated on the heap.
    #[inline]
    pub fn from(s: &str) -> String {
        let mut vec: Vec<u8> = Vec::with_capacity(s.len());
        let mut i = 0;
        while i < s.len() {
            unsafe { vec.push(*s.as_ptr().offset(i as isize)) };
            i += 1;
        }
        String { vec: vec }
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            write!(f, "{}", self.get_char_at(i))?;
        }
        Ok(())
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            write!(f, "{}", self.get_char_at(i))?;
        }
        Ok(())
    }
}