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
