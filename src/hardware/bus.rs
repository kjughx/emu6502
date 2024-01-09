use crate::types::{Addr, Byte};

pub const MEMORY_SIZE: usize = 1024 * 64;
pub const STACK_START: Addr = Addr(0x100);
pub const STACK_END: Addr = Addr(0x01ff);
pub const STACK_SIZE: usize = 0xff;

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
        let mut data = [Byte(0); MEMORY_SIZE];
        data[0x100..0x01ff].copy_from_slice(&[Byte(0xff); STACK_SIZE]);
        Self { data }
    }

    /// Access memory at address @addr
    pub fn read(&self, addr: Addr) -> Byte {
        self.data[addr.0 as usize]
    }
    /// Write @data to memory at address @addr
    pub fn write(&mut self, addr: Addr, data: Byte) {
        self.data[addr.0 as usize] = data;
    }

    pub fn init(&mut self, mut addr: Addr, data: Vec<u8>) {
        for byte in data {
            self.data[addr.0 as usize] = Byte(byte);
            addr.0 += 1;
        }
    }
}
