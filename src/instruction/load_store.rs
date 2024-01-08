use super::InstructionArgument;
use crate::cpu::{Flag, CPU};
use crate::types::Bit;

pub fn lda(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!(),
    };
    cpu.a = val;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.a == 0));
}

pub fn ldx(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!(),
    };
    cpu.x = val;
    cpu.set(Flag::Negative, cpu.x & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.x == 0));
}

pub fn ldy(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!(),
    };
    cpu.y = val;
    cpu.set(Flag::Negative, cpu.y & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.y == 0));
}
