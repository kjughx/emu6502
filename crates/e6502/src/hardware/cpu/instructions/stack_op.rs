use super::InstructionArgument;
use crate::hardware::cpu::{Flag, Register, CPU};
use crate::types::Bit;

pub fn tax(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.x = cpu.a;
    cpu.set(Flag::Negative, cpu.x & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.x == 0));
    true
}
pub fn tay(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.y = cpu.a;
    cpu.set(Flag::Negative, cpu.y & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.y == 0));
    true
}
pub fn txa(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.a = cpu.x;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.a == 0));
    true
}

pub fn tya(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.a = cpu.y;
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.a == 0));
    true
}

pub fn tsx(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.x = cpu.sp;
    cpu.set(Flag::Negative, cpu.x & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.x == 0));
    true
}

pub fn txs(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.sp = cpu.x;
    true
}

pub fn pha(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.push_stack(cpu.a);
    true
}

pub fn php(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.push_stack(cpu.get_reg(Register::PS).unwrap_left() | Flag::Break);
    true
}

pub fn pla(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.a = cpu.pop_stack();
    cpu.set(Flag::Negative, cpu.a & Flag::Negative);
    cpu.set(Flag::Zero, Bit(cpu.a == 0));
    true
}

pub fn plp(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    cpu.ps = cpu.pop_stack();
    cpu.ps &= !Flag::Break;
    true
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_stack() {
        use crate::hardware::cpu::system;
        use crate::hardware::*;

        let mut system = system::System::new().pc(0x400);
        for (i, byte) in include_bytes!("stack.bin").iter().enumerate() {
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
            assert!(instructions <= 185, "Too many instructions!");
        }

        assert_eq!(
            cpu.get_pc(),
            Addr(0x0518),
            "Failure: {:#06X}",
            cpu.get_pc().0
        );
    }
}
