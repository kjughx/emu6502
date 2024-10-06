use crate::types::{Addr, Byte};
use std::path::Path;

use super::bus::Device;

pub const ROM_SIZE: Addr = Addr(0x80ff);
const ADDR_START: Addr = Addr(0x7f00);
const ADDR_END: Addr = Addr(0xffff);

pub struct Rom {
    data: [Byte; (ROM_SIZE.0 + 1) as usize],
}

impl Rom {
    pub fn new(file: Option<String>) -> Rom {
        let mut data = [Byte(0); (ROM_SIZE.0 + 1) as usize];
        if let Some(file) = file {
            let _data = std::fs::read(Path::new(&file)).unwrap();
            for (i, byte) in _data[..((ROM_SIZE.0 - 1) as usize)].iter().enumerate() {
                data[i] = Byte(*byte);
            }
        }

        Self { data }
    }
}

impl Device for Rom {
    fn rx(&mut self, _addr: Addr, _data: Byte) {
        panic!("Not allowed to write to ROM");
    }

    #[allow(clippy::absurd_extreme_comparisons)]
    fn tx(&mut self, addr: Addr) -> Byte {
        assert!(
            addr.0 <= ADDR_START.0 + ROM_SIZE.0,
            "ROM: Outside memory region {:#06X}",
            addr.0
        );
        self.data[(addr.0 - ADDR_START.0) as usize]
    }
    fn range(&self) -> (Addr, Addr) {
        (ADDR_START, ADDR_END)
    }
}
