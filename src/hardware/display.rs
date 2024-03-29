use crate::hardware::bus::Device;
use crate::types::{Addr, Byte};
use std::io::Write;

pub const KEY_DATA: Addr = Addr(0x5002);
pub const KEY_READY: Addr = Addr(0x5003);
pub const READY: Byte = Byte(0x08);

pub struct Display;

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

impl Display {
    pub fn new() -> Self {
        Self
    }
}

impl Device for Display {
    fn rx(&mut self, addr: Addr, data: Byte) {
        if addr == KEY_DATA {
            print!("{}", data.0 as char);
            std::io::stdout().flush().unwrap();
        }
    }
    fn tx(&mut self, addr: Addr) -> Byte {
        match addr {
            KEY_READY => READY,
            _ => Byte(0x00),
        }
    }
}
