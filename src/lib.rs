pub mod hardware;
pub mod instruction;
pub mod types;

#[macro_export]
macro_rules! Mutex(
    ($b:expr) => (
        Arc::new(Mutex::new($b))
    )
);

#[macro_export]
macro_rules! dbg_bin(
    ($b:expr) => (
        println!("{}: {:#09b}", file!(), $b)
    );
);

#[macro_export]
macro_rules! dbg_byte(
    ($b:expr) => (
        println!("{}: {:#04X}", file!(), $b)
    );
);

#[macro_export]
macro_rules! dbg_addr(
    ($a:expr) => (
        println!("{}: {:#06X}", file!(), $b)
    )
);

#[cfg(test)]
mod tests {
    use crate::hardware::*;
    use crate::types::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_functional() {
        let bus = Mutex!(bus::Bus::new());
        let memory = Mutex!(memory::Memory::new(Addr(0xffff)));
        bus.lock()
            .unwrap()
            .register(memory, Addr(0x0000), Addr(0xffff))
            .unwrap();

        for (i, byte) in include_bytes!("routines/6502_functional_test.bin")
            .iter()
            .enumerate()
        {
            bus.lock().unwrap().write(Addr(i as u16), Byte(*byte));
        }

        let cpu = Mutex!(cpu::CPU::new(bus));
        cpu.lock().unwrap().pc = Addr(0x0400);

        let mut instructions = 0;

        loop {
            if !cpu.lock().unwrap().exec() {
                break;
            }
            instructions += 1;
        }

        assert_eq!(
            cpu.lock().unwrap().pc,
            Addr(0x336D),
            "Failure: {:#06X}",
            cpu.lock().unwrap().pc.0
        );
        assert!(instructions == 26765879, "Too many instructions");
    }
}
