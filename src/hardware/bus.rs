use crate::types::{Addr, Byte};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
    ///
    /// Note: The device does not necessarily need to support this.
    #[allow(unused_variables)]
    fn rx(&mut self, addr: Addr, data: Byte) {
        unimplemented!("Not supported for device");
    }

    /// Someone reads from `addr` belonging to this device
    fn tx(&mut self, addr: Addr) -> Byte;

    fn range(&self) -> (Addr, Addr);
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
    ) -> Result<(), String> {
        let (start, end) = dev.lock().unwrap().range();
        for key in self.indices.keys() {
            if *key >= start.0 && *key < end.0 {
                return Err("Overlaping addresses".to_string())?;
            }
        }

        println!("Registering device at {start}..{end}");

        self.devices.push(dev);
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
        dev.lock().unwrap().tx(addr)
    }

    /// Write on bus `data` to address `addr`
    pub fn write(&mut self, addr: Addr, data: Byte) {
        if !self.indices.contains_key(&addr.0) {
            eprintln!("Nothing registered at {:#06X}", addr.0);
            return
        }
        let dev = &self.devices[self.indices[&addr.0]];
        dev.lock().unwrap().rx(addr, data);
    }
}
