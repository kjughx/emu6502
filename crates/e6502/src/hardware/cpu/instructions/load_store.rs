use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::Bit;

pub fn lda(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };
    cpu.a = val;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.a == 0));

    true
}

pub fn ldx(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };
    cpu.x = val;
    cpu.set(Flag::Negative, cpu.x & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.x == 0));

    true
}

pub fn ldy(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let val = match arg {
        InstructionArgument::Immediate(v) => v,
        InstructionArgument::Address(addr) => cpu.read(addr),
        _ => unreachable!("Illegal addressing mode: {:?}", arg),
    };
    cpu.y = val;
    cpu.set(Flag::Negative, cpu.y & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.y == 0));

    true
}

pub fn sta(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };
    cpu.write(addr, cpu.a);
    true
}

pub fn stx(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };
    cpu.write(addr, cpu.x);
    true
}

pub fn sty(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };
    cpu.write(addr, cpu.y);
    true
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_load_store() {
        use crate::hardware::cpu::system;
        use crate::hardware::*;

        let mut system = system::System::new().pc(0x400);
        for (i, byte) in include_bytes!("load_store.bin").iter().enumerate() {
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
            assert!(instructions <= 3334, "Too many instructions!");
        }

        assert_eq!(
            cpu.get_pc(),
            Addr(0x10A5),
            "Failure: {:#06X}",
            cpu.get_pc().0
        );
    }
}
