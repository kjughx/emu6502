use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::*;

pub fn adc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };

    let sum = cpu.a + val + (cpu.ps & Flag::Carry);
    cpu.set(
        Flag::Overflow,
        ((cpu.a & Flag::Negative) & (val & Flag::Negative)) ^ (sum & Flag::Negative),
    );
    cpu.set(Flag::Zero, Bit(sum == 0));
    cpu.set(Flag::Negative, sum & Flag::Negative);
    cpu.set(Flag::Carry, Bit(sum < cpu.a || sum < val));
    cpu.a = sum;

    true
}

pub fn sbc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
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
    cpu.set(Flag::Negative, Bit(cpu.x == val));
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
    cpu.set(Flag::Negative, Bit(cpu.y == val));
    cpu.set(Flag::Negative, Bit(cpu.y < val));

    true
}
