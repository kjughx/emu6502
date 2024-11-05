pub mod bus;
pub mod cpu;
pub mod display;
pub mod keyboard;
pub mod memory;
pub mod rom;
pub mod serial;


use crate::types::{Addr, Byte};

/// Trait for devices on `Bus`
pub trait Device {
    /// Someone writes `data` to `addr` belonging to this device
    ///
    /// Note: The device does not necessarily need to support this.
    #[allow(unused_variables)]
    fn rx(&self, addr: Addr, data: Byte) {
        unimplemented!("Not supported for device");
    }

    /// Someone reads from `addr` belonging to this device
    fn tx(&self, addr: Addr) -> Byte;

    fn range(&self) -> (Addr, Addr);
}

