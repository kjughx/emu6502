use super::cpu::instructions::*;
use crate::hardware::bus::Bus;
use crate::types::*;
use std::fmt::Display;
pub mod instructions;

pub const STACK_START: Addr = Addr(0x100);
pub const STACK_END: Addr = Addr(0x01ff);
pub const STACK_SIZE: usize = 0xff;

/// brief: The processor status flags
///
/// Carry: Set if last operation overflow bit 7 or underflowed bit 0
/// Zero: Set if last operation resulted in zero
/// InterruptDisable: Set if interrupts should be ignored
/// DecimalMode: Not Supported
/// BreakCmd: Set if the BRK instruction was executed
/// Unused: Unused
/// Overflow: Set if last operation yieled incorrect 2's complement
/// Negative: Set if last operation resulted in a negative value
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode,
    Break,
    Reserved,
    Overflow,
    Negative,
}

pub enum Register {
    A,
    X,
    Y,
    PS,
}

///Contains the CPU state
///
///pc: Program Counter
///sp: Stack Pointer
///a:  Acculumator
///x:  Index Register X
///y:  Index Register Y
///ps: Processor Status
pub struct CPU {
    pc: Addr, // Program Counter
    sp: Byte, // Stack Pointer

    a: Byte, // Acculumator
    x: Byte, // Index Register X
    y: Byte, // Index Register Y

    ps: Byte, // Processor Status
    bus: Bus,

    irq_pending: bool,
    nmi_pending: bool,
}

impl CPU {
    /// Create a CPU instance connected on `bus`
    pub fn new(bus: Bus) -> Self {
        Self {
            pc: Addr(0xfffc),
            sp: Byte(0xfd),
            a: Byte(0x00),
            x: Byte(0x00),
            y: Byte(0x00),
            ps: Byte(0x00),
            bus,
            irq_pending: false,
            nmi_pending: false,
        }
    }

    /// Check if `flag` of the processor status is set
    pub fn is_set(&self, flag: Flag) -> bool {
        (self.ps & flag).0
    }

    /// Set `flag` of the processor status to `bit`
    pub fn set(&mut self, flag: Flag, bit: Bit) {
        assert!(!matches!(flag, Flag::Break), "Can't set BREAK flag");
        if bit.0 {
            self.ps |= Byte(1 << (flag as u8));
        } else {
            self.ps &= !Byte(1 << (flag as u8));
        }
    }

    /// Push `data` onto the stack
    ///
    /// Note: This decrements the stack pointer
    pub fn push_stack(&mut self, data: Byte) {
        self.bus.write(STACK_START + self.sp, data);

        if self.sp == 0 {
            self.sp = STACK_END.low()
        } else {
            self.sp -= 1;
        }
    }

    /// Pop `data` from the stack
    ///
    /// Note: This increments the stack pointer
    pub fn pop_stack(&mut self) -> Byte {
        if self.sp == STACK_END & 0xFF {
            self.sp = Byte(0);
        } else {
            self.sp += 1;
        }

        self.bus.read(STACK_START + self.sp)
    }

    /// Read from the `bus` at `addr`
    pub fn read(&self, addr: impl Into<Addr>) -> Byte {
        self.bus.read(addr.into())
    }

    /// Write `data` to the `bus` at `addr`
    pub fn write(&mut self, addr: Addr, data: Byte) {
        assert!(
            addr.0 < 0x0100 || addr.0 > 0x01ff,
            "tried to write to stack at {:#06X}",
            addr.0
        );
        self.bus.write(addr, data)
    }

    /// Emulate a hard reset
    pub fn reset(&mut self) {
        self.x = Byte(0x00);
        self.y = Byte(0x00);
        self.ps = Byte(0x00);
        self.a = Byte(0x00);
        self.sp = Byte(0xfd);

        self.set(Flag::InterruptDisable, Bit(true));

        let low_addr = self.read(0xfffc);
        let hi_addr = self.read(0xfffd);
        self.pc = Addr::new(hi_addr, low_addr);
    }

    /// Handle an interrupt request
    fn irq(&mut self) {
        if self.is_set(Flag::InterruptDisable) {
            return;
        }

        self.push_stack(self.pc.low());
        self.push_stack(self.pc.high());
        self.push_stack(self.ps);

        self.set(Flag::InterruptDisable, Bit(true));

        let low_addr = self.read(0xfffe);
        let hi_addr = self.read(0xffff);
        self.pc = Addr::new(hi_addr, low_addr);
    }

    /// Handle a non-maskable interrupt
    fn nmi_irq(&mut self) {
        self.set(Flag::InterruptDisable, Bit(true));

        self.push_stack(self.pc.low());
        self.push_stack(self.pc.high());
        self.push_stack(self.ps);
        let low_addr = self.read(0xfffa);
        let hi_addr = self.read(0xfffb);
        self.pc = Addr::new(hi_addr, low_addr);
    }

    /// Set a pending interrupt request
    pub fn pend_irq(&mut self) {
        self.irq_pending = true;
    }

    pub fn halt(&self, msg: Option<&'static str>) {
        eprintln!("CPU halted!: {}", msg.unwrap_or_default());
        println!("State of CPU: ");
        println!("{self}");
        std::process::exit(1);
    }

    /// Execute next instruction
    ///
    /// If an interrupt is pending then handle that first
    ///
    /// Note: This increments the program counter
    pub fn exec(&mut self) -> bool {
        if self.nmi_pending {
            self.nmi_irq();
        }

        if self.irq_pending {
            self.irq();
        }

        let (instruction, addressing_mode) = get_instruction(self.read(self.pc));
        let arg = addressing_mode.get(self);

        let prev_pc = self.pc;
        // Move self.pc past the argument of the instruction
        if instruction.exec(arg, self) {
            self.pc += match addressing_mode {
                AddressingMode::Implied => 1,
                AddressingMode::Immediate => 2,
                AddressingMode::ZeroPage => 2,
                AddressingMode::ZeroPageX => 2,
                AddressingMode::ZeroPageY => 2,
                AddressingMode::Indirect => 2,
                AddressingMode::IndirectX => 2,
                AddressingMode::IndirectY => 2,
                AddressingMode::Relative => 2,
                AddressingMode::Absolute => 3,
                AddressingMode::AbsoluteX => 3,
                AddressingMode::AbsoluteY => 3,
            }
        }

        if self.pc == prev_pc {
            println!("TRAP: {:#06X}", prev_pc.0);
            println!("{self}");
            return false;
        }

        true
    }

    /// Read the program counter
    pub fn get_pc(&self) -> Addr {
        self.pc
    }

    // #[cfg(test)]
    pub fn set_pc(&mut self, addr: Addr) {
        self.pc = addr;
    }

    /// Read the stack pointer
    pub fn get_sp(&self) -> Byte {
        self.sp
    }

    /// Read register `reg`
    pub fn get_reg(&self, reg: Register) -> Byte {
        match reg {
            Register::A => self.a,
            Register::X => self.x,
            Register::Y => self.y,
            Register::PS => self.ps | Flag::Reserved, // Flag::Reserved is always set
        }
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\tPC: {:#06x}\n\tSP: {:#02x}\n\ta: {:#02x}, x: {:#02x}, y: {:#02x}\n",
            self.pc.0, self.sp.0, self.a.0, self.x.0, self.y.0
        )?;
        write!(
            f,
            "\tC: {}, Z: {}, I: {}, D: -\n\tB: {}, V: {}, N: {}",
            self.ps & Flag::Carry,
            self.ps & Flag::Zero,
            self.ps & Flag::InterruptDisable,
            self.ps & Flag::Break,
            self.ps & Flag::Overflow,
            self.ps & Flag::Negative
        )
    }
}
