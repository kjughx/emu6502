use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::{Addr, Bit, Byte};

pub fn brk(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.push_stack(Byte::from(cpu.pc >> 8));
    cpu.push_stack(Byte::from(cpu.pc & 0xff) + 2);
    cpu.push_stack(cpu.ps | Flag::Break);
    let low_addr = cpu.read(Addr::from(0xfffe));
    let hi_addr = cpu.read(Addr::from(0xffff));
    cpu.pc = (Addr::from(hi_addr) << 8) | low_addr;
    cpu.set(Flag::InterruptDisable, Bit(true));
    false
}

pub fn rti(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.ps = cpu.pop_stack() & !Flag::Break | Flag::Reserved;
    let low_addr = cpu.pop_stack();
    let hi_addr = cpu.pop_stack();
    cpu.pc = (Addr::from(hi_addr) << 8) | low_addr;
    false
}
