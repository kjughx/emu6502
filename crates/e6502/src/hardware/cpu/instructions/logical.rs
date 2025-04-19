use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::*;

pub fn and(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };
    cpu.a &= val;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.a == 0));

    true
}

pub fn eor(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };
    cpu.a ^= val;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.a == 0));

    true
}

pub fn ora(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };
    cpu.a |= val;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.a == 0));

    true
}

pub fn bit(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg)
    };

    let val = cpu.read(addr);
    cpu.set(Flag::Zero, Bit(val & cpu.a == 0));
    cpu.set(Flag::Negative, val & Flag::Negative);
    cpu.set(Flag::Overflow, val & Flag::Overflow);

    true
}

pub fn cmp(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };

    cpu.set(Flag::Carry, Bit(cpu.a >= val));
    cpu.set(Flag::Zero, Bit(cpu.a == val));
    cpu.set(Flag::Negative, Bit(cpu.a < val));

    true
}

pub fn cpx(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };

    cpu.set(Flag::Carry, Bit(cpu.x >= val));
    cpu.set(Flag::Zero, Bit(cpu.x == val));
    cpu.set(Flag::Negative, Bit(cpu.x < val));

    true
}

pub fn cpy(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };

    cpu.set(Flag::Carry, Bit(cpu.y >= val));
    cpu.set(Flag::Zero, Bit(cpu.y == val));
    cpu.set(Flag::Negative, Bit(cpu.y < val));

    true
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_logical() {
        use crate::hardware::*;

        let mut bus = bus::Bus::new();
        let memory = memory::Memory::new(Addr(0x0000), Addr(0xffff));
        bus.register(memory).unwrap();

        for (i, byte) in include_bytes!("logical.bin").iter().enumerate() {
            bus.write(Addr(i as u16), Byte(*byte));
        }

        let mut cpu = cpu::CPU::new(bus);
        cpu.set_pc(Addr(0x0400));

        let mut instructions = 0;

        loop {
            if !cpu.exec() {
                break;
            }
            instructions += 1;
            assert!(instructions <= 3977, "Too many instructions!");
        }

        assert_eq!(
            cpu.get_pc(),
            Addr(0x10E9),
            "Failure: {:#06X}",
            cpu.get_pc().0
        );
    }
}
