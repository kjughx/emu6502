use super::InstructionArgument;
use crate::hardware::cpu::{Flag, CPU};
use crate::types::{Addr, Bit, Byte};

pub fn jmp(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };
    cpu.pc = addr;

    false
}

pub fn jsr(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    let InstructionArgument::Address(addr) = arg else {
        unreachable!("Illegal addressing mode: {:?}", arg);
    };
    cpu.push_stack(Byte::from(cpu.pc >> 8));
    cpu.push_stack(Byte::from((cpu.pc & 0x00ff) + 2));
    cpu.pc = addr;

    false
}

pub fn rts(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(matches!(arg, InstructionArgument::Implied));
    let low_addr = cpu.pop_stack();
    let hi_addr = cpu.pop_stack();
    cpu.pc = (Addr::from(hi_addr) << 8) | (low_addr + 1);

    false
}

pub fn brk(arg: InstructionArgument, cpu: &mut CPU) -> bool {
    assert!(
        matches!(arg, InstructionArgument::Implied),
        "Illegal addressing mode"
    );
    cpu.push_stack(Byte::from(cpu.pc >> 8));
    cpu.push_stack(Byte::from(cpu.pc & 0xff) + 2);
    cpu.push_stack(cpu.ps | Flag::Break | Flag::Reserved);
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
    cpu.ps = cpu.pop_stack() & !Flag::Break;
    let low_addr = cpu.pop_stack();
    let hi_addr = cpu.pop_stack();
    cpu.pc = (Addr::from(hi_addr) << 8) | low_addr;
    false
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_jump_call() {
        use crate::hardware::*;
        use crate::mutex;
        use std::sync::{Arc, Mutex};

        let mut bus = bus::Bus::new();
        let memory = memory::Memory::new(Addr(0x0000), Addr(0xffff));
        bus.register(memory).unwrap();

        for (i, byte) in include_bytes!("jump_call.bin").iter().enumerate() {
            bus.write(Addr(i as u16), Byte(*byte));
        }

        let mut cpu = cpu::CPU::new(mutex!(bus));
        cpu.set_pc(Addr(0x0400));

        let mut instructions = 0;

        loop {
            if !cpu.exec() {
                break;
            }
            instructions += 1;
            assert!(instructions <= 341, "Too many instructions!");
        }

        assert_eq!(
            cpu.get_pc(),
            Addr(0x0533),
            "Failure: {:#06X}",
            cpu.get_pc().0
        );
    }
}
