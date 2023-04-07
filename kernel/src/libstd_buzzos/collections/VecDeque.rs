use crate::{libstd_buzzos::memory::Box::Box, println};
use core::ptr::{null_mut};
use crate::{libstd_buzzos::syscalls::{syscall1, Sysno}};

pub struct Node<T : Copy> {
    pub value: T,
    pub next: *mut Node<T>,
    pub prev: *mut Node<T>,
}

impl<T : Copy> Node<T> {
    pub unsafe fn new(value: T) -> *mut Node<T> {
        println!("Trying to alloc");
        println!("{:#?}", core::mem::size_of::<Node<T>>());
        let pointer = unsafe{syscall1(Sysno::Sbrk,core::mem::size_of::<Node<T>>()) as *mut Node<T>};
        (*pointer).value = value;
        (*pointer).next = null_mut();
        (*pointer).next = null_mut();

        pointer
    }
}

pub struct VecDeque<T : Copy> {
    pub head: *mut Node<T>,
    pub tail: *mut Node<T>,
    pub len: usize,
}


impl<T : Copy> VecDeque<T> {
    pub fn new() -> Self {
        VecDeque {
            head: null_mut(),
            tail: null_mut(),
            len: 0,
        }
    }

    pub unsafe fn push_front(&mut self, value: T) {
        let mut new_node: *mut Node<T> = Node::new(value); 
        (*new_node).prev = null_mut();
        (*new_node).next = self.head;

        if self.head == null_mut() {
            self.head = new_node;
            self.tail = new_node;
        } else {
            (*self.head).prev = new_node;
            self.head = new_node;
        }

        self.len += 1;
    }

    pub unsafe fn push_back(&mut self, value: T) {
        let mut new_node: *mut Node<T> = Node::new(value); 
        (*new_node).next = null_mut();
        (*new_node).prev = self.tail;

        if self.head == null_mut() {
            self.head = new_node;
            self.tail = new_node;
        } else {
            (*self.tail).next = new_node;
            self.tail = new_node;
        }

        self.len += 1;
    }

    pub unsafe fn pop_front(&mut self) -> Option<T> {
        if self.head == null_mut() {
            panic!("ERROR: popping from an empty list.");
        }

        let value = (*self.head).value;
        self.head = (*self.head).next;
        (*self.head).prev = null_mut();
        
        if (*self.head).next == null_mut() {
            self.tail = self.head; 
        }

        self.len -= 1;

        return Some(value);
    }

    pub unsafe fn pop_back(&mut self) -> Option<T> {
        if self.head == null_mut() {
            panic!("ERROR: popping from an empty list.");
        }

        let value = (*self.tail).value;
        self.tail = (*self.tail).prev;
        (*self.tail).prev = null_mut();
        
        if (*self.tail).prev == null_mut() {
            self.head = null_mut();
        }

        self.len -= 1;

        return Some(value);
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
