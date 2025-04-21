use super::cpu::instructions::*;
use crate::hardware::bus::Bus;
use crate::types::*;
use std::collections::HashSet;
use std::fmt::Display;
pub mod instructions;
use crate::hardware::clock::Clock;
use either::Either;
use std::sync::Arc;

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

#[derive(Debug, Copy, Clone)]
pub enum Register {
    A,
    X,
    Y,
    PS,
    SP,
    PC,
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

    previous_pc: Addr,
    advance: bool,
    reset: bool,
    trap: bool,

    clk: Arc<Clock>,
    irq_pending: bool,
    nmi_pending: bool,

    // For debugger
    debug: bool,
    breakpoints: Option<HashSet<Addr>>,
    breaked: bool,
}

impl CPU {
    /// Create a CPU instance connected on `bus`
    pub fn new(bus: Bus, clk: Arc<Clock>) -> Self {
        Self {
            pc: Addr(0xfffc),
            sp: Byte(0xfd),
            a: Byte(0x00),
            x: Byte(0x00),
            y: Byte(0x00),
            ps: Byte(0x00),
            previous_pc: Addr(0x0000),
            bus,
            advance: false,
            reset: true,
            trap: false,
            debug: false,
            breakpoints: None,
            breaked: false,
            clk,
            irq_pending: false,
            nmi_pending: false,
        }
    }

    pub fn debug(&mut self) {
        self.debug = true;
    }

    pub fn breakpoint(&mut self, bp: Addr) {
        if self.breakpoints.is_none() {
            self.breakpoints = Some(HashSet::new());
        };

        self.breakpoints.as_mut().unwrap().insert(bp);
    }

    fn with_tick<F, U>(&self, f: F) -> U
    where
        F: Fn(&Self) -> U,
    {
        if self.debug {
            return f(self);
        }

        self.clk.wait_tick();
        let r = f(self);
        self.clk.tock();

        r
    }

    fn with_tick_mut<F, U>(&mut self, f: F) -> U
    where
        F: Fn(&mut Self) -> U,
    {
        if self.debug {
            return f(self);
        }
        self.clk.wait_tick();
        let r = f(self);
        self.clk.tock();

        r
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
        self.with_tick_mut(|cpu| cpu.bus.write(STACK_START + cpu.sp, data));

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

        self.with_tick(|cpu| cpu.bus.read(STACK_START + cpu.sp))
    }

    /// Read from the `bus` at `addr`
    pub fn read(&self, addr: impl Into<Addr> + Copy) -> Byte {
        self.with_tick(move |cpu| cpu.bus.read(addr.into()))
    }

    fn next_pc(&self) -> Addr {
        if self.advance {
            self.pc + 1
        } else {
            self.pc
        }
    }

    fn read_pc(&mut self) -> Byte {
        if self.advance {
            self.pc += 1;
        }
        self.advance = true;
        self.read(self.pc)
    }

    /// Write `data` to the `bus` at `addr`
    pub fn write(&mut self, addr: Addr, data: Byte) {
        assert!(
            addr.0 < 0x0100 || addr.0 > 0x01ff,
            "tried to write to stack at {:#06X}",
            addr.0
        );
        self.with_tick_mut(move |cpu| cpu.bus.write(addr, data))
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
        self.reset = false;
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
    pub fn trap(&mut self) {
        self.trap = true;
    }

    /// Execute next instruction
    ///
    /// If an interrupt is pending then handle that first
    ///
    /// Note: This increments the program counter
    pub fn exec(&mut self) -> bool {
        assert!(!self.reset);
        if self.trap {
            return false;
        }

        if self.nmi_pending {
            self.nmi_irq();
        }

        if self.irq_pending {
            self.irq();
        }

        let npc = self.next_pc();

        let (instruction, addressing_mode) = self.fetch_decode();
        let arg = self.fetch_argument(addressing_mode);
        instruction.exec(arg, self);

        if self.next_pc() == npc {
            println!("CPU TRAPPED\n{self}");
            self.trap();
            return false;
        }

        true
    }

    pub fn debug_exec(&mut self) -> bool {
        if self.trap {
            println!("CPU TRAPPED\n{self}");
            return false;
        }

        let npc = self.next_pc();
        if !self.breaked {
            if let Some(ref breakpoints) = self.breakpoints {
                if breakpoints.get(&npc).is_some() {
                    self.breaked = true;
                    return false;
                }
            }
        }

        self.breaked = false;

        let (instruction, addressing_mode) = self.fetch_decode();
        let arg = self.fetch_argument(addressing_mode);
        // println!("{}: {instruction}, {arg}", self.previous_pc);
        instruction.exec(arg, self);

        if self.next_pc() == npc {
            println!("CPU TRAPPED\n{self}");
            self.trap();
            return false;
        }

        true
    }

    fn fetch_decode(&mut self) -> (Instruction, AddressingMode) {
        self.previous_pc = self.next_pc();

        let op_code = self.read_pc();
        if OPCODES.contains_key(&op_code) {
            OPCODES[&op_code.0]
        } else if matches!(
            op_code.0,
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2
        ) {
            (Instruction::XXX(op_code.0), AddressingMode::Implied)
        } else {
            (Instruction::NOP, AddressingMode::Implied)
        }
    }

    fn fetch_argument(&mut self, mode: AddressingMode) -> InstructionArgument {
        match mode {
            AddressingMode::Immediate => InstructionArgument::Immediate(self.read_pc()),
            AddressingMode::ZeroPage => InstructionArgument::Address(Addr::from(self.read_pc())),
            AddressingMode::ZeroPageX => {
                InstructionArgument::Address(Addr::from(self.read_pc() + self.x))
            }
            AddressingMode::ZeroPageY => {
                InstructionArgument::Address(Addr::from(self.read_pc() + self.y))
            }
            AddressingMode::Absolute => {
                let low_addr = self.read_pc();
                let hi_addr = self.read_pc();
                InstructionArgument::Address(Addr::new(hi_addr, low_addr))
            }
            AddressingMode::AbsoluteX => {
                let low_addr = self.read_pc();
                let hi_addr = self.read_pc();
                InstructionArgument::Address(Addr::new(hi_addr, low_addr) + self.x)
            }
            AddressingMode::AbsoluteY => {
                let low_addr = self.read_pc();
                let hi_addr = self.read_pc();
                InstructionArgument::Address(Addr::new(hi_addr, low_addr) + self.y)
            }
            AddressingMode::Indirect => {
                let low_addr = self.read_pc();
                let hi_addr = self.read_pc();
                let _addr = Addr::new(hi_addr, low_addr);
                InstructionArgument::Address(
                    (Addr::from(self.read(_addr + 1)) << 8) | self.read(_addr),
                )
            }
            AddressingMode::IndirectX => {
                let _addr = Addr::from(self.read_pc() + self.x);
                InstructionArgument::Address(
                    (Addr::from(self.read(_addr + 1)) << 8) | self.read(_addr),
                )
            }
            AddressingMode::IndirectY => {
                let _addr = Addr::from(self.read_pc());
                let low_addr = self.read(_addr);
                let hi_addr = self.read(_addr + 1);
                InstructionArgument::Address(Addr::new(hi_addr, low_addr) + self.y)
            }
            AddressingMode::Relative => InstructionArgument::Offset(self.read_pc()),
            AddressingMode::Implied => InstructionArgument::Implied,
        }
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
    pub fn get_reg(&self, reg: Register) -> Either<Byte, Addr> {
        match reg {
            Register::A => Either::Left(self.a),
            Register::X => Either::Left(self.x),
            Register::Y => Either::Left(self.y),
            Register::PS => Either::Left(self.ps | Flag::Reserved), // Flag::Reserved is always set
            Register::SP => Either::Left(self.sp),
            Register::PC => {
                if self.advance {
                    Either::Right(self.pc + 1)
                } else {
                    Either::Right(self.pc)
                }
            }
        }
    }

    pub fn set_reg(&mut self, reg: Register, val: Either<Byte, Addr>) {
        assert!(self.debug);
        match reg {
            Register::A => self.a = val.unwrap_left(),
            Register::X => self.x = val.unwrap_left(),
            Register::Y => self.y = val.unwrap_left(),
            Register::PS => self.ps = val.unwrap_left(),
            Register::SP => self.sp = val.unwrap_left(),
            Register::PC => self.pc = val.unwrap_right(),
        };
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

pub mod system {
    use crate::hardware::*;
    use std::sync::Arc;

    #[derive(Default)]
    struct Registers {
        a: Option<u8>,
        x: Option<u8>,
        y: Option<u8>,
        ps: Option<u8>,
    }

    #[derive(Default)]
    pub struct System {
        memory: Vec<(u16, u8)>,
        regs: Registers,
        pc: Option<u16>,
        sp: Option<u8>,
    }

    #[allow(dead_code)]
    impl System {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn memory(mut self, addr: u16, val: u8) -> Self {
            self.memory.push((addr, val));
            self
        }
        pub fn set_memory(&mut self, addr: u16, val: u8) {
            self.memory.push((addr, val));
        }

        pub fn a(mut self, a: u8) -> Self {
            self.regs.a = Some(a);
            self
        }

        pub fn x(mut self, x: u8) -> Self {
            self.regs.x = Some(x);
            self
        }

        pub fn y(mut self, y: u8) -> Self {
            self.regs.y = Some(y);
            self
        }

        pub fn ps(mut self, ps: u8) -> Self {
            self.regs.ps = Some(ps);
            self
        }
        pub fn pc(mut self, pc: u16) -> Self {
            self.pc = Some(pc);
            self
        }

        pub fn sp(mut self, sp: u8) -> Self {
            self.sp = Some(sp);
            self
        }

        pub fn prepare(self) -> (cpu::CPU, Arc<clock::Clock>) {
            let mut bus = bus::Bus::new();
            let memory = memory::Memory::new(Addr(0x0000), Addr(0xffff));
            bus.register(memory).unwrap();
            for (addr, val) in self.memory {
                bus.write(addr, val);
            }

            let clk = std::sync::Arc::new(clock::Clock::new());
            let mut cpu = cpu::CPU::new(bus, clk.clone());
            let _clk = clk.clone();
            std::thread::spawn(move || {
                for _ in 0..2 {
                    _clk.tick();
                    _clk.wait_tock();
                }
            });
            cpu.reset();

            if let Some(a) = self.regs.a {
                cpu.a = a.into();
            }
            if let Some(x) = self.regs.x {
                cpu.x = x.into();
            }
            if let Some(y) = self.regs.y {
                cpu.y = y.into();
            }
            if let Some(ps) = self.regs.ps {
                cpu.ps = ps.into();
            }
            if let Some(pc) = self.pc {
                cpu.pc = pc.into();
            }
            if let Some(sp) = self.sp {
                cpu.sp = sp.into();
            }

            (cpu, clk)
        }
    }
}
