use crate::hardware::Device;
use crate::types::{Addr, Byte};
use std::collections::HashMap;

// Encapsulates the 16-bit wide bus
pub struct Bus {
    devices: Vec<Box<dyn Device>>,
    indices: HashMap<u16, usize>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Self {
            devices: vec![],
            indices: HashMap::new(),
        }
    }

    pub fn register<T: Device + 'static>(&mut self, dev: T) -> Result<(), String> {
        let (start, end) = dev.range();
        for key in self.indices.keys() {
            if *key >= start.0 && *key < end.0 {
                return Err("Overlaping addresses".to_string())?;
            }
        }

        println!("Registering device at {start}..{end}");

        self.devices.push(Box::new(dev));
        let len = self.devices.len();
        for addr in start.0..=end.0 {
            self.indices.insert(addr, len - 1);
        }

        Ok(())
    }

    /// Read on bus from address `addr`
    pub fn read(&self, addr: Addr) -> Byte {
        if !self.indices.contains_key(&addr.0) {
            panic!("Nothing registered at {:#06X}", addr.0);
        }
        let dev = &self.devices[self.indices[&addr.0]];
        dev.tx(addr)
    }

    /// Write on bus `data` to address `addr`
    pub fn write(&mut self, addr: impl Into<Addr>, data: impl Into<Byte>) {
        let addr = addr.into();
        let data = data.into();
        if !self.indices.contains_key(&addr.0) {
            eprintln!("Nothing registered at {:#06X}", addr.0);
            return;
        }
        let dev = &mut self.devices[self.indices[&addr.0]];
        dev.rx(addr, data);
    }
}
