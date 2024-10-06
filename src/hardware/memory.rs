use crate::types::{Addr, Byte};

use super::bus::Device;

const ADDR_START: Addr = Addr(0x0000);
const ADDR_END: Addr = Addr(0x3fff);
pub const MEMORY_SIZE: usize = 0x4000;

pub struct Memory {
    data: Vec<Byte>,
    start: Addr,
    end: Addr,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(ADDR_START, ADDR_END)
    }
}

impl Memory {
    pub fn new(start: Addr, end: Addr) -> Self {
        let size = (end - start).0 as usize + 1;
        Self {
            data: vec![Byte(0); size],
            start,
            end,
        }
    }
}

impl Device for Memory {
    fn rx(&mut self, addr: Addr, data: Byte) {
        assert!(
            addr <= self.end,
            "Outside memory region: {:#06X}",
            addr.0
        );
        self.data[addr.0 as usize] = data;
    }
    fn tx(&mut self, addr: Addr) -> Byte {
        assert!(
            addr <= self.end,
            "Outside memory region: {:#06X}",
            addr.0
        );
        self.data[addr.0 as usize]
    }
    fn range(&self) -> (Addr, Addr) {
        (self.start, self.end)
    }
}
