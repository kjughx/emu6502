use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::*;

pub fn adc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };

    let sum: u16 = (cpu.a.0 as u16) + (val.0 as u16) + (cpu.ps & Flag::Carry).0 as u16;

    cpu.set(Flag::Carry, Bit(sum > 0xff));
    let sum: Byte = Byte((sum & 0xff) as u8);

    cpu.set(
        Flag::Overflow,
        (!(cpu.a ^ val) & (cpu.a ^ sum)) & Flag::Negative,
    );
    cpu.set(Flag::Zero, Bit(sum == 0));
    cpu.set(Flag::Negative, sum & Flag::Negative);

    cpu.a = sum;

    true
}

pub fn sbc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };

    let val = !val;

    let sub: u16 = (cpu.a.0 as u16) + (val.0 as u16) + (cpu.ps & Flag::Carry).0 as u16;

    cpu.set(Flag::Carry, Bit(sub > 0xff));
    let sub: Byte = Byte((sub & 0xff) as u8);

    cpu.set(
        Flag::Overflow,
        (!(cpu.a ^ val) & (cpu.a ^ sub)) & Flag::Negative,
    );
    cpu.set(Flag::Zero, Bit(sub == 0));
    cpu.set(Flag::Negative, sub & Flag::Negative);

    cpu.a = sub;

    true
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_arithmetic() {
        use crate::hardware::*;
        use crate::mutex;
        use std::sync::{Arc, Mutex};

        let mut bus = bus::Bus::new();
        let memory = memory::Memory::new(Addr(0x0000), Addr(0xffff));
        bus.register(memory).unwrap();

        for (i, byte) in include_bytes!("arithmetic.bin").iter().enumerate() {
            bus.write(Addr(i as u16), Byte(*byte));
        }

        let mut cpu = cpu::CPU::new(mutex!(bus));
        cpu.set_pc(Addr(0x0400));

        let mut instructions = 0;

        loop {
            if !cpu.exec() {
                break;
            }
            instructions += 1;
            assert!(instructions <= 26709519, "Too many instructions!");
        }

        assert_eq!(
            cpu.get_pc(),
            Addr(0x0459),
            "Failure: {:#06X}",
            cpu.get_pc().0
        );
    }
}
