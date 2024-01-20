use crate::types::{Addr, Byte};

use super::bus::Device;

pub const MEMORY_SIZE: Addr = Addr(0x4000);
pub const MEMORY_START: Addr = Addr(0x0000);

pub struct Memory {
    data: Vec<Byte>,
    size: Addr,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(MEMORY_SIZE)
    }
}

impl Memory {
    pub fn new(size: Addr) -> Self {
        Self {
            data: vec![Byte(0); size.0 as usize + 1],
            size,
        }
    }
}

impl Device for Memory {
    fn rx(&mut self, addr: Addr, data: Byte) {
        assert!(addr <= MEMORY_START + self.size, "Outside memory region");
        self.data[addr.0 as usize] = data;
    }
    fn tx(&mut self, addr: Addr) -> Byte {
        assert!(addr <= MEMORY_START + self.size, "Outside memory region");
        self.data[addr.0 as usize]
    }
}
