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
    cpu.set(Flag::Negative, val & Flag::Negative);

    true
}

pub fn inx(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.x += 1;
    cpu.set(Flag::Zero, Bit(cpu.x == 0));
    cpu.set(Flag::Negative, cpu.x & Flag::Negative);

    true
}

pub fn iny(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.y += 1;
    cpu.set(Flag::Zero, Bit(cpu.y == 0));
    cpu.set(Flag::Negative, cpu.y & Flag::Negative);

    true
}

pub fn dec(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg)
    };

    let val = cpu.read(addr) - 1;

    cpu.write(addr, val);
    cpu.set(Flag::Zero, Bit(val == 0));
    cpu.set(Flag::Negative, val & Flag::Negative);

    true
}

pub fn dex(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.x -= 1;
    cpu.set(Flag::Zero, Bit(cpu.x == 0));
    cpu.set(Flag::Negative, cpu.x & Flag::Negative);

    true
}

pub fn dey(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.y -= 1;
    cpu.set(Flag::Zero, Bit(cpu.y == 0));
    cpu.set(Flag::Negative, cpu.y & Flag::Negative);

    true
}
