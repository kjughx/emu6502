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

#[cfg(test)]
mod test {
    #[test]
    pub fn test_increment_decrement() {
        use crate::hardware::cpu::system;
        use crate::hardware::*;

        let mut system = system::System::new().pc(0x400);
        for (i, byte) in include_bytes!("increment_decrement.bin").iter().enumerate() {
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
            assert!(instructions <= 2069, "Too many instructions!");
        }

        assert_eq!(
            cpu.get_pc(),
            Addr(0xA17),
            "Failure: {:#06X}",
            cpu.get_pc().0
        );
    }
}
