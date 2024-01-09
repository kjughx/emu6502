use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::{Bit, Byte};

pub fn asl(arg: InstructionArgument, cpu: &mut CPU) {
    match arg {
        InstructionArgument::Implied => {
            cpu.set(Flag::Carry, cpu.a & Flag::Negative);
            cpu.a <<= 1;
            cpu.set(Flag::Zero, Bit(cpu.a == 0));
            cpu.set(Flag::Negative, cpu.a & Flag::Negative);
        }
        InstructionArgument::Address(addr) => {
            let mut val = cpu.read_memory(addr);
            cpu.set(Flag::Carry, val & Flag::Negative);
            val <<= 1;
            cpu.set(Flag::Zero, Bit(val == 0));
            cpu.set(Flag::Negative, val & Flag::Negative);
            cpu.write_memory(addr, val);
        }
        _ => unreachable!("Illegal addressing mode: {:?}", arg)
    }
}

pub fn lsr(arg: InstructionArgument, cpu: &mut CPU) {
    match arg {
        InstructionArgument::Implied => {
            cpu.set(Flag::Carry, cpu.a & Flag::Carry);
            cpu.a >>= 1;
            cpu.set(Flag::Zero, Bit(cpu.a == 0));
            cpu.set(Flag::Negative, cpu.a & Flag::Negative);
        }
        InstructionArgument::Address(addr) => {
            let mut val = cpu.read_memory(addr);
            cpu.set(Flag::Carry, val & Flag::Carry);
            val >>= 1;
            cpu.set(Flag::Zero, Bit(val == 0));
            cpu.set(Flag::Negative, val & Flag::Negative);
            cpu.write_memory(addr, val);
        }
        _ => unreachable!("Illegal addressing mode: {:?}", arg)
    }
}

pub fn rol(arg: InstructionArgument, cpu: &mut CPU) {
    match arg {
        InstructionArgument::Implied => {
            let carry = cpu.is_set(Flag::Carry) as u8;
            cpu.set(Flag::Carry, cpu.a & Flag::Negative);
            cpu.a = Byte((cpu.a << 1) | carry);
            cpu.set(Flag::Zero, Bit(cpu.a == 0));
            cpu.set(Flag::Negative, cpu.a & Flag::Negative);
        }
        InstructionArgument::Address(addr) => {
            let carry = cpu.is_set(Flag::Carry) as u8;
            let mut val = cpu.read_memory(addr);
            cpu.set(Flag::Carry, val & Flag::Negative);
            val = Byte((val << 1) | carry);
            cpu.set(Flag::Zero, Bit(val == 0));
            cpu.set(Flag::Negative, val & Flag::Negative);
            cpu.write_memory(addr, val);
        }
        _ => unreachable!("Illegal addressing mode: {:?}", arg)
    }
}

pub fn ror(arg: InstructionArgument, cpu: &mut CPU) {
    match arg {
        InstructionArgument::Implied => {
            let carry = cpu.is_set(Flag::Carry) as u8;
            cpu.set(Flag::Carry, cpu.a & Flag::Carry);
            cpu.a = Byte((cpu.a << 1) | (carry << 7));
            cpu.set(Flag::Zero, Bit(cpu.a == 0));
            cpu.set(Flag::Negative, cpu.a & Flag::Negative);
        }
        InstructionArgument::Address(addr) => {
            let carry = cpu.is_set(Flag::Carry) as u8;
            let mut val = cpu.read_memory(addr);
            cpu.set(Flag::Carry, val & Flag::Carry);
            val = Byte((val << 1) | (carry << 7));
            cpu.set(Flag::Zero, Bit(val == 0));
            cpu.set(Flag::Negative, val & Flag::Negative);
            cpu.write_memory(addr, val);
        }
        _ => unreachable!("Illegal addressing mode: {:?}", arg)
    }
}
