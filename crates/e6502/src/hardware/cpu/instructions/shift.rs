use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::Bit;

pub fn asl(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    match arg {
        InstructionArgument::Implied => {
            cpu.set(Flag::Carry, cpu.a & Flag::Negative);
            cpu.a <<= 1;
            cpu.set(Flag::Zero, Bit(cpu.a == 0));
            cpu.set(Flag::Negative, cpu.a & Flag::Negative);
        }
        InstructionArgument::Address(addr) => {
            let mut val = cpu.read(addr);
            cpu.set(Flag::Carry, val & Flag::Negative);
            val <<= 1;
            cpu.set(Flag::Zero, Bit(val == 0));
            cpu.set(Flag::Negative, val & Flag::Negative);
            cpu.write(addr, val);
        }
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    }

    true
}

pub fn lsr(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    match arg {
        InstructionArgument::Implied => {
            cpu.set(Flag::Carry, cpu.a & Flag::Carry);
            cpu.a >>= 1;
            cpu.set(Flag::Zero, Bit(cpu.a == 0));
            cpu.set(Flag::Negative, cpu.a & Flag::Negative);
        }
        InstructionArgument::Address(addr) => {
            let mut val = cpu.read(addr);
            cpu.set(Flag::Carry, val & Flag::Carry);
            val >>= 1;
            cpu.set(Flag::Zero, Bit(val == 0));
            cpu.set(Flag::Negative, val & Flag::Negative);
            cpu.write(addr, val);
        }
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    }

    true
}

pub fn rol(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    match arg {
        InstructionArgument::Implied => {
            let carry = cpu.is_set(Flag::Carry) as u8;
            cpu.set(Flag::Carry, cpu.a & Flag::Negative);
            cpu.a = (cpu.a << 1) | carry;
            cpu.set(Flag::Zero, Bit(cpu.a == 0));
            cpu.set(Flag::Negative, cpu.a & Flag::Negative);
        }
        InstructionArgument::Address(addr) => {
            let carry = cpu.is_set(Flag::Carry) as u8;
            let mut val = cpu.read(addr);
            cpu.set(Flag::Carry, val & Flag::Negative);
            val = (val << 1) | carry;
            cpu.set(Flag::Zero, Bit(val == 0));
            cpu.set(Flag::Negative, val & Flag::Negative);
            cpu.write(addr, val);
        }
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    }

    true
}

pub fn ror(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    match arg {
        InstructionArgument::Implied => {
            let carry = cpu.ps & Flag::Carry;
            cpu.set(Flag::Carry, cpu.a & Flag::Carry);
            cpu.a = (cpu.a >> 1) | (carry << Flag::Negative);
            cpu.set(Flag::Zero, Bit(cpu.a == 0));
            cpu.set(Flag::Negative, cpu.a & Flag::Negative);
        }
        InstructionArgument::Address(addr) => {
            let carry = cpu.ps & Flag::Carry;
            let mut val = cpu.read(addr);
            cpu.set(Flag::Carry, val & Flag::Carry);
            val = (val >> 1) | (carry << Flag::Negative);
            cpu.set(Flag::Zero, Bit(val == 0));
            cpu.set(Flag::Negative, val & Flag::Negative);
            cpu.write(addr, val);
        }
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    }

    true
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_shift() {
        use crate::hardware::cpu::system;
        use crate::hardware::*;

        let mut system = system::System::new().pc(0x400);
        for (i, byte) in include_bytes!("shift.bin").iter().enumerate() {
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
            assert!(instructions <= 5676, "Too many instructions!");
        }

        assert_eq!(
            cpu.get_pc(),
            Addr(0x0B50),
            "Failure: {:#06X}",
            cpu.get_pc().0
        );
    }
}
