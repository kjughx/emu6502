use super::InstructionArgument;
use crate::cpu::{Flag, CPU};
use crate::types::*;

pub fn adc(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!(),
    };

    let sum = cpu.a + val + (cpu.ps & Flag::Carry);
    cpu.set(
        Flag::Overflow,
        ((cpu.a & Flag::Negative) & (val & Flag::Negative)) ^ (sum & Flag::Negative),
    );
    cpu.set(Flag::Zero, Bit::from(sum));
    cpu.set(Flag::Negative, sum & Flag::Negative);
    cpu.set(Flag::Carry, Bit(sum < cpu.a || sum < val));
    cpu.a = sum;
}

pub fn sbc(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!(),
    };
    let sub = cpu.a + (val & !(0xff)) + (cpu.ps & Flag::Carry);
    cpu.set(
        Flag::Overflow,
        ((cpu.a & Flag::Negative) & (val & Flag::Negative)) ^ (sub & Flag::Negative),
    );
    cpu.set(Flag::Zero, Bit::from(sub));
    cpu.set(Flag::Negative, sub & Flag::Negative);
    cpu.set(Flag::Carry, Bit(sub < cpu.a || sub < val));
    cpu.a = sub;
}

pub fn cmp(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!(),
    };

    cpu.set(Flag::Carry, Bit(cpu.a >= val));
    cpu.set(Flag::Negative, Bit(cpu.a == val));
    cpu.set(Flag::Negative, Bit(cpu.a < val));
}

pub fn cpx(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!(),
    };

    cpu.set(Flag::Carry, Bit(cpu.x >= val));
    cpu.set(Flag::Negative, Bit(cpu.x == val));
    cpu.set(Flag::Negative, Bit(cpu.x < val));
}

pub fn cpy(arg: InstructionArgument, cpu: &mut CPU) {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read_memory(addr),
        _ => unreachable!(),
    };

    cpu.set(Flag::Carry, Bit(cpu.y >= val));
    cpu.set(Flag::Negative, Bit(cpu.y == val));
    cpu.set(Flag::Negative, Bit(cpu.y < val));
}
