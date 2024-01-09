use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::*;

pub fn and(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg)
    };
    cpu.a &= val;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(!(cpu.a == 0)));
}

pub fn eor(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg)
    };
    cpu.a ^= val;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(!(cpu.a == 0)));
}

pub fn ora(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg)
    };
    cpu.a |= val;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.a == 0));
}

pub fn bit(arg: InstructionArgument, cpu: &mut CPU) {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg)
    };

    let val = cpu.a & cpu.read_memory(addr);
    cpu.set(Flag::Negative, val & Flag::Negative);
    cpu.set(Flag::Overflow, val & Flag::Overflow);
    cpu.set(Flag::Zero, Bit(val == 0));
}
