use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::Bit;

pub fn clc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.set(Flag::Carry, Bit(false));
    true
}

pub fn cld(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.set(Flag::DecimalMode, Bit(false));
    true
}

pub fn cli(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.set(Flag::InterruptDisable, Bit(false));
    true
}

pub fn clv(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.set(Flag::Overflow, Bit(false));
    true
}

pub fn sec(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.set(Flag::Carry, Bit(true));
    true
}

pub fn sed(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.set(Flag::DecimalMode, Bit(true));
    true
}

pub fn sei(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.set(Flag::InterruptDisable, Bit(true));
    true
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_flag() {
        use crate::hardware::*;
        use crate::types::*;
        use crate::Mutex;
        use std::sync::{Arc, Mutex};

        let bus = Mutex!(bus::Bus::new());
        let memory = Mutex!(memory::Memory::new(Addr(0xffff)));
        bus.lock()
            .unwrap()
            .register(memory, Addr(0x0000), Addr(0xffff))
            .unwrap();

        for (i, byte) in include_bytes!("flag.bin").iter().enumerate() {
            bus.lock().unwrap().write(Addr(i as u16), Byte(*byte));
        }

        let cpu = Mutex!(cpu::CPU::new(bus));
        cpu.lock().unwrap().set_pc(Addr(0x0400));

        let mut instructions = 0;

        loop {
            if !cpu.lock().unwrap().exec() {
                break;
            }
            instructions += 1;
            assert!(instructions <= 119, "Too many instructions!");
        }

        assert_eq!(
            cpu.lock().unwrap().get_pc(),
            Addr(0x049A),
            "Failure: {:#06X}",
            cpu.lock().unwrap().get_pc().0
        );
    }
}
