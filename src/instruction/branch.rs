use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};

pub fn bcc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if !cpu.is_set(Flag::Carry) {
        if (offset & Flag::Negative).0 {
            cpu.pc = cpu.pc - (!*offset + 1) + 2;
        } else {
            cpu.pc = cpu.pc + offset + 2;
        }
        return false;
    }
    true
}

pub fn bcs(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if cpu.is_set(Flag::Carry) {
        if (offset & Flag::Negative).0 {
            cpu.pc = cpu.pc - (!*offset + 1) + 2;
        } else {
            cpu.pc = cpu.pc + offset + 2;
        }
        return false;
    }
    true
}

pub fn beq(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if cpu.is_set(Flag::Zero) {
        if (offset & Flag::Negative).0 {
            cpu.pc = cpu.pc - (!*offset + 1) + 2;
        } else {
            cpu.pc = cpu.pc + offset + 2;
        }
        return false;
    }
    true
}

pub fn bmi(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if cpu.is_set(Flag::Negative) {
        if (offset & Flag::Negative).0 {
            cpu.pc = cpu.pc - (!*offset + 1) + 2;
        } else {
            cpu.pc = cpu.pc + offset + 2;
        }
        return false;
    }
    true
}

pub fn bne(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if !cpu.is_set(Flag::Zero) {
        if (offset & Flag::Negative).0 {
            cpu.pc = cpu.pc - (!*offset + 1) + 2;
        } else {
            cpu.pc = cpu.pc + offset + 2;
        }
        return false;
    }
    true
}

pub fn bpl(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if !cpu.is_set(Flag::Negative) {
        if (offset & Flag::Negative).0 {
            cpu.pc = cpu.pc - (!*offset + 1) + 2;
        } else {
            cpu.pc = cpu.pc + offset + 2;
        }
        return false;
    }
    true
}

pub fn bvc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if !cpu.is_set(Flag::Overflow) {
        if (offset & Flag::Negative).0 {
            cpu.pc = cpu.pc - (!*offset + 1) + 2;
        } else {
            cpu.pc = cpu.pc + offset + 2;
        }
        return false;
    }
    true
}

pub fn bvs(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if cpu.is_set(Flag::Overflow) {
        if (offset & Flag::Negative).0 {
            cpu.pc = cpu.pc - (!*offset + 1) + 2;
        } else {
            cpu.pc = cpu.pc + offset + 2;
        }
        return false;
    }
    true
}
