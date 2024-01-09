use super::bus::{Bus, STACK_END, STACK_START};
use crate::instruction::{AddressingMode, Instruction};
use crate::types::*;
use std::fmt::Display;

/// @brief: The processor status flags
///
/// @Carry: Set if last operation overflow bit 7 or underflowed bit 0
/// @Zero: Set if last operation resulted in zero
/// @InterruptDisable: Set if interrupts should be ignored
/// @DecimalMode: Not Supported
/// @BreakCmd: Set if the BRK instruction was executed
/// @Unused: Unused
/// @Overflow: Set if last operation yieled incorrect 2's complement
/// @Negative: Set if last operation resulted in a negative value
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode,
    BreakCmd,
    Unused,
    Overflow,
    Negative,
}

///@brief: Contains the CPU state
///
///@pc: Program Counter
///@sp: Stack Pointer
///@a:  Acculumator
///@x:  Index Register X
///@y:  Index Register Y
///@ps: Processor Status
pub struct CPU<'a> {
    pub pc: Addr, // Program Counter
    pub sp: Byte, // Stack Pointer

    pub a: Byte, // Acculumator
    pub x: Byte, // Index Register X
    pub y: Byte, // Index Register Y

    pub ps: Byte, // Processor Status
    bus: &'a mut Bus,
}

impl<'a> CPU<'a> {
    pub fn new(bus: &'a mut Bus) -> Self {
        Self {
            pc: Addr(0xfffc),
            sp: Byte(0xfd),
            a: Byte(0x00),
            x: Byte(0x00),
            y: Byte(0x00),
            ps: Byte(0x00),
            bus,
        }
    }

    pub fn is_set(&self, flag: Flag) -> bool {
        (self.ps & flag).0
    }

    pub fn set(&mut self, flag: Flag, bit: Bit) {
        if bit.0 {
            self.ps |= Byte(1 << (flag as u8));
            dbg!("Setting flag", flag, self.ps);
        } else {
            self.ps &= !Byte(1 << (flag as u8));
            dbg!("Clearing flag", flag, self.ps);
        }
    }

    pub fn push_stack(&mut self, data: Byte) {
        self.bus.write(STACK_START + self.sp, data);

        if self.sp == 0 {
            self.sp = Byte::from(STACK_END & 0xFF);
        } else {
            self.sp -= 1;
        }
    }

    pub fn pop_stack(&mut self) -> Byte {
        if self.sp == STACK_END & 0xFF {
            self.sp = Byte(0);
        } else {
            self.sp += 1;
        }

        self.bus.read(STACK_START + self.sp)
    }

    pub fn next_instruction(&mut self) -> Byte {
        let val = self.read_memory(self.pc);
        self.pc += 1;

        val
    }

    pub fn read_memory(&self, addr: Addr) -> Byte {
        self.bus.read(addr)
    }

    pub fn write_memory(&mut self, addr: Addr, data: Byte) {
        self.bus.write(addr, data)
    }

    pub fn reset(&mut self) {
        self.ps = Byte::from(0x00);
        self.a = Byte::from(0x00);
        self.sp = Byte::from(0xfd);
        let low_addr = self.read_memory(Addr::from(0xfffc));
        let hi_addr = Addr::from(self.read_memory(Addr::from(0xfffd)));
        self.pc = (hi_addr << 8) | low_addr;
    }
    pub fn irq(&mut self) {
        if self.is_set(Flag::InterruptDisable) {
            return;
        }

        self.set(Flag::InterruptDisable, Bit(true));
        self.set(Flag::BreakCmd, Bit(false));

        self.push_stack(Byte::from(self.pc & 0x00ff));
        self.push_stack(Byte::from(self.pc & 0xff00));
        self.push_stack(self.ps);
        let low_addr = self.read_memory(Addr::from(0xfffe));
        let hi_addr = self.read_memory(Addr::from(0xffff));
        self.pc = (Addr::from(hi_addr) << 8) | low_addr;
    }

    pub fn nmi_irq(&mut self) {
        self.set(Flag::InterruptDisable, Bit(true));
        self.set(Flag::BreakCmd, Bit(false));

        self.push_stack(Byte::from(self.pc & 0x00ff));
        self.push_stack(Byte::from(self.pc & 0xff00));
        self.push_stack(self.ps);
        let low_addr = self.read_memory(Addr::from(0xfffa));
        let hi_addr = self.read_memory(Addr::from(0xfffb));
        self.pc = (Addr::from(hi_addr) << 8) | low_addr;
    }

    pub fn halt(&self) {
        eprintln!("CPU halted!");
        println!("State of CPU: ");
        println!("{self}");
    }

    pub fn exec(&mut self) {
        let op_code = self.next_instruction();
        let arg = AddressingMode::from(op_code).get(self);
        let instruction = Instruction::from(op_code);

        instruction.exec(arg, self);
    }
}

impl Display for CPU<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\tPC: {:#06x}\n\tSP: {:#02x}\n\ta: {:#02x}, x: {:#02x}, y: {:#02x}\n\tPS: {:#010b}",
            self.pc.0, self.sp.0, self.a.0, self.x.0, self.y.0, self.ps.0
        )
    }
}
