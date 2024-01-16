use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::Bit;

pub fn inc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg)
    };

    let val = cpu.read(addr) + 1;

    cpu.write(addr, val);
    cpu.set(Flag::Zero, Bit(val == 0));

    true
}

pub fn inx(_arg: InstructionArgument, cpu: &mut CPU) -> bool {
    cpu.x += 1;
    cpu.set(Flag::Zero, Bit(cpu.x == 0));

    true
}

pub fn iny(_arg: InstructionArgument, cpu: &mut CPU) -> bool {
    cpu.y += 1;
    cpu.set(Flag::Zero, Bit(cpu.y == 0));

    true
}

pub fn dec(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg)
    };

    let val = cpu.read(addr) - 1;

    cpu.write(addr, val);
    cpu.set(Flag::Zero, Bit(val == 0));

    true
}

pub fn dex(_arg: InstructionArgument, cpu: &mut CPU) -> bool {
    cpu.x -= 1;
    cpu.set(Flag::Zero, Bit(cpu.x == 0));

    true
}

pub fn dey(_arg: InstructionArgument, cpu: &mut CPU) -> bool {
    cpu.y -= 1;
    cpu.set(Flag::Zero, Bit(cpu.y == 0));

    true
}
