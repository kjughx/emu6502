use super::InstructionArgument;
use crate::hardware::cpu::CPU;
use crate::types::{Addr, Byte};

pub fn jmp(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };
    cpu.pc = addr;

    false
}

pub fn jsr(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };
    cpu.push_stack(Byte::from(cpu.pc >> 8));
    cpu.push_stack(Byte::from((cpu.pc & 0x00ff) + 2));
    cpu.pc = addr;

    false
}

pub fn rts(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    let low_addr = cpu.pop_stack();
    let hi_addr = cpu.pop_stack();
    cpu.pc = (Addr::from(hi_addr) << 8) | (low_addr + 1);

    false
}
