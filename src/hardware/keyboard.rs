use crate::types::{Addr, Byte};
use console::Term;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use super::bus::Device;

pub const KEY_DATA: Addr = Addr(0x5000);
pub const KEY_READY: Addr = Addr(0x5001);
pub const READY: Byte = Byte(0x08);
pub const NOT_READY: Byte = Byte(0x00);

pub struct Keyboard {
    data: VecDeque<u8>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Self {
            data: vec![].into(),
        }
    }
}

impl Device for Keyboard {
    fn tx(&mut self, addr: Addr) -> Byte {
        match addr {
            KEY_READY => {
                if !self.data.is_empty() {
                    READY
                } else {
                    NOT_READY
                }
            }
            KEY_DATA => {
                if !self.data.is_empty() {
                    Byte(self.data.pop_front().unwrap())
                } else {
                    Byte(0x00)
                }
            }
            _ => unreachable!("Unsupported read :{addr:?}"),
        }
    }
}

pub fn poll(keyboard: Arc<Mutex<Keyboard>>) {
    let term = Term::stdout();
    loop {
        let c = match term.read_char() {
            Err(_) => continue,
            Ok(c) => c,
        };
        keyboard.lock().unwrap().data.push_back(c as u8);
    }
}
