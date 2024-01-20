use crate::types::{Addr, Byte};

use super::bus::Device;

pub const MEMORY_SIZE: Addr = Addr(0x4000);
pub const MEMORY_START: Addr = Addr(0x0000);

pub struct Memory {
    data: [Byte; MEMORY_SIZE.0 as usize + 1],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [Byte(0); MEMORY_SIZE.0 as usize + 1],
        }
    }
}

impl Device for Memory {
    fn rx(&mut self, addr: Addr, data: Byte) {
        assert!(
            addr.0 <= MEMORY_START.0 + MEMORY_SIZE.0,
            "Outside memory region"
        );
        self.data[addr.0 as usize] = data;
    }
    fn tx(&mut self, addr: Addr) -> Byte {
        assert!(
            addr.0 <= MEMORY_START.0 + MEMORY_SIZE.0,
            "Outside memory region"
        );
        self.data[addr.0 as usize]
    }
}
