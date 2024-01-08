use crate::types::{Addr, Byte};

const MEMORY_SIZE: usize = 1024 * 64;
pub const STACK_START: Addr = Addr(0x100);
pub const STACK_END: Addr = Addr(0x01ff);

#[derive(Debug)]
pub struct Bus {
    pub data: [Byte; MEMORY_SIZE],
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Self {
            data: [Byte(0); MEMORY_SIZE],
        }
    }

    /// Access memory at address @addr
    pub fn read(&self, addr: Addr) -> Byte {
        self.data[addr.0 as usize]
    }
    /// Write @data to memory at address @addr
    pub fn write(&mut self, addr: Addr, data: Byte) {
        self.data[addr.0 as usize] = data;
    }
}
