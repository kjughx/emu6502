use crate::types::{Addr, Byte};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub const STACK_START: Addr = Addr(0x100);
pub const STACK_END: Addr = Addr(0x01ff);
pub const STACK_SIZE: usize = 0xff;

/// Encapsulates the 16-bit wide bus
pub struct Bus {
    devices: Vec<Arc<Mutex<dyn Device>>>,
    indices: HashMap<u16, usize>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for devices on `Bus`
pub trait Device: Send + Sync {
    /// Someone writes `data` to `addr` belonging to this device
    fn rx(&mut self, _addr: Addr, _data: Byte) {}

    /// Someone reads from `addr` belonging to this device
    fn tx(&mut self, addr: Addr) -> Byte;
}

impl Bus {
    pub fn new() -> Self {
        Self {
            devices: vec![],
            indices: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        dev: Arc<Mutex<dyn Device>>,
        start: Addr,
        end: Addr,
    ) -> Result<(), String> {
        for key in self.indices.keys() {
            if *key >= start.0 && *key < end.0 {
                return Err("Overlaping addresses".to_string())?;
            }
        }

        self.devices.push(dev);
        let len = self.devices.len();
        for _addr in start.0..=end.0 {
            self.indices.insert(_addr, len - 1);
        }

        Ok(())
    }

    /// Read on bus from address `addr`
    pub fn read(&self, addr: Addr) -> Byte {
        if !self.indices.contains_key(&addr.0) {
            println!("Nothing registered at {:#06X}", addr.0);
            return Byte(0x00);
        }
        let dev = &self.devices[self.indices[&addr.0]];
        dev.lock().unwrap().tx(addr)
    }

    /// Write on bus `data` to address `addr`
    pub fn write(&mut self, addr: Addr, data: Byte) {
        let dev = &self.devices[self.indices[&addr.0]];
        dev.lock().unwrap().rx(addr, data);
    }
}
