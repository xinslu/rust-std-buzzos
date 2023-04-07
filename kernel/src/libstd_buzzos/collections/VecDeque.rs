use crate::libstd_buzzos::memory::Box::Box;
use core::ptr::{null_mut};

struct Node<T : Copy> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T : Copy> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: null_mut(),
            prev: null_mut(),
        }
    }
}

struct Deque<T : Copy> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}


impl<T : Copy> Deque<T> {
    fn new() -> Self {
        Deque {
            head: null_mut(),
            tail: null_mut(),
            len: 0,
        }
    }

    unsafe fn push_front(&mut self, value: T) {
        let mut new_node: *mut Node<T> = Box::new(Node::new(value)) as *mut Node<T>;
        (*new_node).prev = null_mut();
        (*new_node).next = self.head;

        if self.head == null_mut() {
            self.head = new_node;
            self.tail = new_node;
        } else {
            (*self.head).prev = new_node;
        }

        self.len += 1;
    }

    unsafe fn push_back(&mut self, value: T) {
        let mut new_node: *mut Node<T> = Box::new(Node::new(value)) as *mut Node<T>;
        (*new_node).next = null_mut();
        (*new_node).prev = self.tail;

        if self.head == null_mut() {
            self.head = new_node;
            self.tail = new_node;
        } else {
            (*self.tail).next = new_node;
        }

        self.len += 1;
    }

    unsafe fn pop_front(&mut self) -> Option<T> {
        if self.head == null_mut() {
            panic!("Skill issue. Why'd you pop front on an empty deque? Switch majors.");
        }

        let value = (*self.head).value;
        self.head = (*self.head).next;
        (*self.head).prev = null_mut();
        
        if (*self.head).next == null_mut() {
            self.tail = null_mut();
        }

        return Some(value);
    }

    unsafe fn pop_back(&mut self) -> Option<T> {
        if self.head == null_mut() {
            panic!("Skill issue. Why'd you pop back on an empty deque? Switch majors.");
        }

        let value = (*self.tail).value;
        self.tail = (*self.tail).prev;
        (*self.tail).prev = null_mut();
        
        if (*self.tail).prev == null_mut() {
            self.head = null_mut();
        }

        return Some(value);
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
