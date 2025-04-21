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

#[cfg(test)]
mod test {
    use crate::hardware::*;

    #[test]
    pub fn test_arithmetic() {
        use crate::hardware::cpu::system;
        use crate::hardware::*;

        let mut system = system::System::new().pc(0x400);
        for (i, byte) in include_bytes!("arithmetic.bin").iter().enumerate() {
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
            Addr(0x0459),
            "Failure: {:#06X}",
            cpu.get_pc().0
        );
    }

    use crate::hardware::cpu::system;

    #[test]
    fn test_adc() {
        let (mut cpu, clk) = system::System::new()
            .memory(0x0400, 0x69)
            .memory(0x0401, 0x34)
            .pc(0x400)
            .a(0x35)
            .prepare();

        std::thread::spawn(move || loop {
            clk.tick();
            clk.wait_tock();
        });

        cpu.exec();
        assert_eq!(cpu.a, Byte(0x69));
        assert_eq!(cpu.clk.ticks(), 4);
    }
}
