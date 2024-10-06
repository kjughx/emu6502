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

#[cfg(test)]
mod test {
    #[test]
    pub fn test_branch() {
        use crate::hardware::*;
        use crate::types::*;
        use crate::Mutex;
        use std::sync::{Arc, Mutex};

        let bus = Mutex!(bus::Bus::new());
        let memory = Mutex!(memory::Memory::new(Addr(0x0000), Addr(0xffff)));
        bus.lock()
            .unwrap()
            .register(memory)
            .unwrap();

        for (i, byte) in include_bytes!("branch.bin").iter().enumerate() {
            bus.lock().unwrap().write(Addr(i as u16), Byte(*byte));
        }

        let cpu = Mutex!(cpu::CPU::new(bus));
        cpu.lock().unwrap().set_pc(Addr(0x0400));

        let mut instructions = 0;

        loop {
            if !cpu.lock().unwrap().exec() {
                break;
            }
            instructions += 1;
            assert!(instructions <= 40423, "Too many instructions!");
        }

        assert_eq!(
            cpu.lock().unwrap().get_pc(),
            Addr(0x0718),
            "Failure: {:#06X}",
            cpu.lock().unwrap().get_pc().0
        );
    }
}
