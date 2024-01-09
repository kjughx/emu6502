use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::Bit;

pub fn inc(arg: InstructionArgument, cpu: &mut CPU) {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg)
    };

    let val = cpu.read_memory(addr) + 1;

    cpu.write_memory(addr, val);
    cpu.set(Flag::Zero, Bit(val == 0));
}

pub fn inx(_arg: InstructionArgument, cpu: &mut CPU) {
    cpu.x += 1;
    cpu.set(Flag::Zero, Bit(cpu.x == 0));
}

pub fn iny(_arg: InstructionArgument, cpu: &mut CPU) {
    cpu.y += 1;
    cpu.set(Flag::Zero, Bit(cpu.y == 0));
}

pub fn dec(arg: InstructionArgument, cpu: &mut CPU) {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg)
    };

    let val = cpu.read_memory(addr) - 1;

    cpu.write_memory(addr, val);
    cpu.set(Flag::Zero, Bit(val == 0));
}

pub fn dex(_arg: InstructionArgument, cpu: &mut CPU) {
    cpu.x -= 1;
    cpu.set(Flag::Zero, Bit(cpu.x == 0));
}

pub fn dey(_arg: InstructionArgument, cpu: &mut CPU) {
    cpu.y -= 1;
    cpu.set(Flag::Zero, Bit(cpu.y == 0));
}
