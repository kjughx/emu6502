use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::Byte;

fn offset_pc(cpu: &mut CPU, offset: Byte) {
    if (offset & Flag::Negative).0 {
        if (!*offset + 1) == 2 {
            cpu.trap();
        }
        cpu.pc = cpu.pc - (!*offset + 1);
    } else {
        cpu.pc = cpu.pc + offset;
    }
}

pub fn bcc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if !cpu.is_set(Flag::Carry) {
        offset_pc(cpu, offset);

        return false;
    }
    true
}

pub fn bcs(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if cpu.is_set(Flag::Carry) {
        offset_pc(cpu, offset);

        return false;
    }
    true
}

pub fn beq(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    println!("Zero is {}, arg: {}", cpu.is_set(Flag::Zero), arg);
    if cpu.is_set(Flag::Zero) {
        offset_pc(cpu, offset);

        return false;
    }

    true
}

pub fn bmi(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if cpu.is_set(Flag::Negative) {
        offset_pc(cpu, offset);

        return false;
    }
    true
}

pub fn bne(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if !cpu.is_set(Flag::Zero) {
        offset_pc(cpu, offset);

        return false;
    }
    true
}

pub fn bpl(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if !cpu.is_set(Flag::Negative) {
        offset_pc(cpu, offset);

        return false;
    }
    true
}

pub fn bvc(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if !cpu.is_set(Flag::Overflow) {
        offset_pc(cpu, offset);

        return false;
    }
    true
}

pub fn bvs(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Offset(offset) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };

    if cpu.is_set(Flag::Overflow) {
        offset_pc(cpu, offset);

        return false;
    }
    true
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_branch() {
        use crate::hardware::cpu::system;
        use crate::hardware::*;

        let mut system = system::System::new().pc(0x400);
        for (i, byte) in include_bytes!("branch.bin").iter().enumerate() {
            system.set_memory(i as u16, *byte);
        }

        let (mut cpu, clk) = system.prepare();

        let mut instructions = 0;
        std::thread::spawn(move || loop {
            clk.tick();
            clk.wait_tock();
        });

        loop {
            if !cpu.exec() {
                break;
            }
            instructions += 1;
            assert!(instructions <= 26709519, "Too many instructions!");
        }

        assert_eq!(
            cpu.get_pc(),
            Addr(0x0718),
            "Failure: {:#06X}",
            cpu.get_pc().0
        );
    }
}
