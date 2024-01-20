use super::bus::{Bus, STACK_END, STACK_START};
use crate::instruction::{get_instruction, AddressingMode};
use crate::types::*;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

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
    Break,
    Reserved,
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
pub struct CPU {
    pub pc: Addr, // Program Counter
    pub sp: Byte, // Stack Pointer

    pub a: Byte, // Acculumator
    pub x: Byte, // Index Register X
    pub y: Byte, // Index Register Y

    pub ps: Byte, // Processor Status
    bus: Arc<Mutex<Bus>>,

    _irq_pending: bool,
    _nmi_pending: bool,
}

impl CPU {
    pub fn new(bus: Arc<Mutex<Bus>>) -> Self {
        Self {
            pc: Addr(0xfffc),
            sp: Byte(0xfd),
            a: Byte(0x00),
            x: Byte(0x00),
            y: Byte(0x00),
            ps: Byte(0x00),
            bus,
            _irq_pending: false,
            _nmi_pending: false,
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

        self.ps |= Flag::Reserved; // Always set
    }

    /// Push `data` onto the stack
    ///
    /// Note: This decrements `self.sp`
    pub fn push_stack(&mut self, data: Byte) {
        self.bus.lock().unwrap().write(STACK_START + self.sp, data);

        if self.sp == 0 {
            self.sp = Byte::from(STACK_END & 0xFF);
        } else {
            self.sp -= 1;
        }
    }

    /// Pop `data` from the stack
    ///
    /// Note: This decrements `self.sp`
    pub fn pop_stack(&mut self) -> Byte {
        if self.sp == STACK_END & 0xFF {
            self.sp = Byte(0);
        } else {
            self.sp += 1;
        }

        self.bus.lock().unwrap().read(STACK_START + self.sp)
    }

    /// Read from `self.bus` at `addr`
    pub fn read(&self, addr: Addr) -> Byte {
        self.bus.lock().unwrap().read(addr)
    }

    /// Write `data` to `self.bus` at `addr`
    pub fn write(&mut self, addr: Addr, data: Byte) {
        assert!(
            addr.0 < 0x0100 || addr.0 > 0x01ff,
            "tried to write to stack at {:#06X}",
            addr.0
        );
        self.bus.lock().unwrap().write(addr, data)
    }

    /// Emulate a hard reset
    pub fn reset(&mut self) {
        self.x = Byte(0x00);
        self.y = Byte(0x00);
        self.ps = Byte(0x00);
        self.a = Byte(0x00);
        self.sp = Byte(0xfd);

        self.set(Flag::InterruptDisable, Bit(true));

        let low_addr = self.read(Addr::from(0xfffc));
        let hi_addr = Addr::from(self.read(Addr::from(0xfffd)));
        self.pc = (hi_addr << 8) | low_addr;
    }

    /// Handle an interrupt request
    pub fn irq(&mut self) {
        if self.is_set(Flag::InterruptDisable) {
            return;
        }

        self.push_stack(Byte::from(self.pc & 0x00ff));
        self.push_stack(Byte::from(self.pc & 0xff00));
        self.push_stack(self.ps);

        self.set(Flag::InterruptDisable, Bit(true));

        let low_addr = self.read(Addr::from(0xfffe));
        let hi_addr = self.read(Addr::from(0xffff));
        self.pc = (Addr::from(hi_addr) << 8) | low_addr;
    }

    /// Handle a non-maskable interrupt
    pub fn nmi_irq(&mut self) {
        self.set(Flag::InterruptDisable, Bit(true));

        self.push_stack(Byte::from(self.pc & 0x00ff));
        self.push_stack(Byte::from(self.pc & 0xff00));
        self.push_stack(self.ps);
        let low_addr = self.read(Addr::from(0xfffa));
        let hi_addr = self.read(Addr::from(0xfffb));
        self.pc = (Addr::from(hi_addr) << 8) | low_addr;
    }

    /// Set `self._irq_pending`
    pub fn pend_irq(&mut self) {
        self._irq_pending = true;
    }

    /// Clear `self._irq_pending`
    pub fn unpend_irq(&mut self) {
        self._irq_pending = false;
    }

    pub fn halt(&self, msg: Option<&'static str>) {
        eprintln!("CPU halted!: {}", msg.unwrap_or_default());
        println!("State of CPU: ");
        println!("{self}");
        std::process::exit(1);
    }

    /// Execute next instruction
    ///
    /// If an interrupt is pending then handle
    /// those first
    ///
    /// Note: This increments `self.pc`
    pub fn exec(&mut self) -> bool {
        if self._nmi_pending {
            self.nmi_irq();
        }

        if self._irq_pending {
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

        if self.pc == Addr(0x0400) {
            println!("SUCCESS");
            std::process::exit(0);
        }

        true
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
