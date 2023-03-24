use core::fmt;
use alloc::{string::{String, ToString}};
use lazy_static::lazy_static;
use spin::Mutex;


use super::uart::{uart_put_char, uart_read_char};

pub struct Console;

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        return Ok(());
    }
}

impl Console {
    fn write_char(&self, c: char) {
        uart_put_char(c).expect("[ERROR] Failed to stream char");
    }

    pub fn write_string(&self, text: &str) {
        for c in text.chars() {
            self.write_char(c);
        }
    }

    fn read_char(&self) -> Result<char, ()> {
        return uart_read_char();
    }
    
    pub fn read_string(&self) -> Result<String, ()> {
        let mut res:String = "".to_string();
        loop {
            match self.read_char() {
                Ok(c) => res = res + (c.encode_utf8(&mut [0; 1]) as &mut str),
                Err(()) => break,
            };
        }
        return Ok(res);
    }
}

lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console {});
}
