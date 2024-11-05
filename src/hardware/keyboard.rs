use crate::types::{Addr, Byte};
use console::Term;
use std::cell::UnsafeCell;
use std::collections::VecDeque;

use super::Device;

pub const KEY_DATA: Addr = Addr(0x5000);
pub const KEY_READY: Addr = Addr(0x5001);
pub const READY: Byte = Byte(0x08);
pub const NOT_READY: Byte = Byte(0x00);

const ADDR_START: Addr = Addr(0x5000);
const ADDR_END: Addr = Addr(0x5001);

pub struct Keyboard {
    data: UnsafeCell<VecDeque<u8>>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Self {
            data: UnsafeCell::new(VecDeque::new()),
        }
    }

    pub fn poll(keyboard: &Keyboard) {
        let term = Term::stdout();
        loop {
            let c = match term.read_char() {
                Err(_) => continue,
                Ok(c) => c,
            };
            let data = unsafe { &mut *keyboard.data.get() };
            data.push_back(c as u8);
        }
    }
}

impl Device for Keyboard {
    fn tx(&self, addr: Addr) -> Byte {
        let data = unsafe { &mut *self.data.get() };
        match addr {
            KEY_READY => {
                if !data.is_empty() {
                    READY
                } else {
                    NOT_READY
                }
            }
            KEY_DATA => {
                if !data.is_empty() {
                    Byte(data.pop_front().unwrap())
                } else {
                    Byte(0x00)
                }
            }
            _ => unreachable!("Unsupported read :{addr:?}"),
        }
    }

    fn range(&self) -> (Addr, Addr) {
        (ADDR_START, ADDR_END)
    }
}
