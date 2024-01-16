pub mod arithmetic;
pub mod increment_decrement;
pub mod load_store;
pub mod logical;
pub mod shift;

use std::fmt::Display;

use phf::phf_map;

use crate::hardware::cpu::{Flag, CPU};
use crate::types::*;

#[derive(Debug)]
pub enum InstructionArgument {
    Immediate(Byte),
    Offset(Byte),
    Address(Addr),
    Implied,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Implied,
}

impl AddressingMode {
    pub fn get(self, cpu: &CPU) -> InstructionArgument {
        match self {
            AddressingMode::Immediate => InstructionArgument::Immediate(cpu.read(cpu.pc + 1)),
            AddressingMode::ZeroPage => {
                InstructionArgument::Address(Addr::from(cpu.read(cpu.pc + 1)))
            }
            AddressingMode::ZeroPageX => {
                InstructionArgument::Address(Addr::from(cpu.read(cpu.pc + 1) + cpu.x))
            }
            AddressingMode::ZeroPageY => {
                InstructionArgument::Address(Addr::from(cpu.read(cpu.pc + 1) + cpu.y))
            }
            AddressingMode::Absolute => {
                let low_addr = cpu.read(cpu.pc + 1);
                let hi_addr = cpu.read(cpu.pc + 2);
                InstructionArgument::Address(Addr::from(hi_addr) << 8 | low_addr)
            }
            AddressingMode::AbsoluteX => {
                let low_addr = cpu.read(cpu.pc + 1);
                let hi_addr = cpu.read(cpu.pc + 2);
                InstructionArgument::Address(((Addr::from(hi_addr) << 8) | low_addr) + cpu.x)
            }
            AddressingMode::AbsoluteY => {
                let low_addr = cpu.read(cpu.pc + 1);
                let hi_addr = cpu.read(cpu.pc + 2);
                InstructionArgument::Address(((Addr::from(hi_addr) << 8) | low_addr) + cpu.y)
            }
            AddressingMode::Indirect => {
                let low_addr = cpu.read(cpu.pc + 1);
                let hi_addr = cpu.read(cpu.pc + 2);
                let _addr = ((Addr::from(hi_addr) << 8) | low_addr) + cpu.y;
                InstructionArgument::Address(
                    Addr::from((cpu.read(_addr)) << 8) | cpu.read(_addr + 1),
                )
            }
            AddressingMode::IndirectX => {
                let _lo = cpu.read(cpu.pc + 1) + cpu.x;
                let _hi = cpu.read(cpu.pc + 2);
                InstructionArgument::Address(Addr::from(_hi << 8) | _lo)
            }
            AddressingMode::IndirectY => {
                let _addr = cpu.read(cpu.pc + 1);
                dbg!(_addr, cpu.y);
                let low_addr = cpu.read(_addr.into());
                let hi_addr = cpu.read((_addr + 1).into());
                InstructionArgument::Address(((Addr::from(hi_addr) << 8) | low_addr) + cpu.y)
            }
            AddressingMode::Relative => InstructionArgument::Offset(cpu.read(cpu.pc + 1)),
            AddressingMode::Implied => InstructionArgument::Implied,
        }
    }
}

impl Display for InstructionArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            InstructionArgument::Implied => "*".to_string(),
            InstructionArgument::Offset(byte) => format!("%{:#04X}", byte.0),
            InstructionArgument::Address(addr) => format!("${:#06X}", addr.0),
            InstructionArgument::Immediate(byte) => format!("#{:#04X}", byte.0),
        };

        write!(f, "{txt}")
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Instruction {
    ADC, AND, ASL, BCC,
    BCS, BEQ, BIT, BMI,
    BNE, BPL, BRK, BVC,
    BVS, CLC, CLD, CLI,
    CLV, CMP, CPX, CPY,
    DEC, DEX, DEY, EOR,
    INC, INX, INY, JMP,
    JSR, LDA, LDX, LDY,
    LSR, NOP, ORA, PHA,
    PHP, PLA, PLP, ROL,
    ROR, RTI, RTS, SBC,
    SEC, SED, SEI, STA,
    STX, STY, TAX, TAY,
    TSX, TXA, TXS, TYA,
    XXX,
}

#[rustfmt::skip]
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            Instruction::ADC => "ADC", Instruction::AND => "AND",
            Instruction::ASL => "ASL", Instruction::BCC => "BCC",
            Instruction::BCS => "BCS", Instruction::BEQ => "BEQ",
            Instruction::BIT => "BIT", Instruction::BMI => "BMI",
            Instruction::BNE => "BNE", Instruction::BPL => "BPL",
            Instruction::BRK => "BRK", Instruction::BVC => "BVC",
            Instruction::BVS => "BVS", Instruction::CLC => "CLC",
            Instruction::CLD => "CLD", Instruction::CLI => "CLI",
            Instruction::CLV => "CLV", Instruction::CMP => "CMP",
            Instruction::CPX => "CPX", Instruction::CPY => "CPY",
            Instruction::DEC => "DEC", Instruction::DEX => "DEX",
            Instruction::DEY => "DEY", Instruction::EOR => "EOR",
            Instruction::INC => "INC", Instruction::INX => "INX",
            Instruction::INY => "INY", Instruction::JMP => "JMP",
            Instruction::JSR => "JSR", Instruction::LDA => "LDA",
            Instruction::LDX => "LDX", Instruction::LDY => "LDY",
            Instruction::LSR => "LSR", Instruction::NOP => "NOP",
            Instruction::ORA => "ORA", Instruction::PHA => "PHA",
            Instruction::PHP => "PHP", Instruction::PLA => "PLA",
            Instruction::PLP => "PLP", Instruction::ROL => "ROL",
            Instruction::ROR => "ROR", Instruction::RTI => "RTI",
            Instruction::RTS => "RTS", Instruction::SBC => "SBC",
            Instruction::SEC => "SEC", Instruction::SED => "SED",
            Instruction::SEI => "SEI", Instruction::STA => "STA",
            Instruction::STX => "STX", Instruction::STY => "STY",
            Instruction::TAX => "TAX", Instruction::TAY => "TAY",
            Instruction::TSX => "TSX", Instruction::TXA => "TXA",
            Instruction::TXS => "TXS", Instruction::TYA => "TYA",
            Instruction::XXX => "XXX",
        };
        write!(f, "{txt}")
    }
}

#[rustfmt::skip]
impl From<&str> for Instruction {
    fn from(val: &str) -> Self {
        match val {
            "ADC" => Instruction::ADC, "AND" => Instruction::AND,
            "ASL" => Instruction::ASL, "BCC" => Instruction::BCC,
            "BCS" => Instruction::BCS, "BEQ" => Instruction::BEQ,
            "BIT" => Instruction::BIT, "BMI" => Instruction::BMI,
            "BNE" => Instruction::BNE, "BPL" => Instruction::BPL,
            "BRK" => Instruction::BRK, "BVC" => Instruction::BVC,
            "BVS" => Instruction::BVS, "CLC" => Instruction::CLC,
            "CLD" => Instruction::CLD, "CLI" => Instruction::CLI,
            "CLV" => Instruction::CLV, "CMP" => Instruction::CMP,
            "CPX" => Instruction::CPX, "CPY" => Instruction::CPY,
            "DEC" => Instruction::DEC, "DEX" => Instruction::DEX,
            "DEY" => Instruction::DEY, "EOR" => Instruction::EOR,
            "INC" => Instruction::INC, "INX" => Instruction::INX,
            "INY" => Instruction::INY, "JMP" => Instruction::JMP,
            "JSR" => Instruction::JSR, "LDA" => Instruction::LDA,
            "LDX" => Instruction::LDX, "LDY" => Instruction::LDY,
            "LSR" => Instruction::LSR, "NOP" => Instruction::NOP,
            "ORA" => Instruction::ORA, "PHA" => Instruction::PHA,
            "PHP" => Instruction::PHP, "PLA" => Instruction::PLA,
            "PLP" => Instruction::PLP, "ROL" => Instruction::ROL,
            "ROR" => Instruction::ROR, "RTI" => Instruction::RTI,
            "RTS" => Instruction::RTS, "SBC" => Instruction::SBC,
            "SEC" => Instruction::SEC, "SED" => Instruction::SED,
            "SEI" => Instruction::SEI, "STA" => Instruction::STA,
            "STX" => Instruction::STX, "STY" => Instruction::STY,
            "TAX" => Instruction::TAX, "TAY" => Instruction::TAY,
            "TSX" => Instruction::TSX, "TXA" => Instruction::TXA,
            "TXS" => Instruction::TXS, "TYA" => Instruction::TYA,
            _ => Instruction::XXX,
        }
    }
}

impl Instruction {
    pub fn exec(&self, arg: InstructionArgument, cpu: &mut CPU) -> bool {
        match self {
            Instruction::LDA => load_store::lda(arg, cpu),
            Instruction::LDX => load_store::ldx(arg, cpu),
            Instruction::LDY => load_store::ldy(arg, cpu),
            Instruction::STA => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                cpu.write(addr, cpu.a);
                true
            }
            Instruction::STX => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                cpu.write(addr, cpu.x);
                true
            }
            Instruction::STY => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                cpu.write(addr, cpu.y);
                true
            }
            Instruction::TAX => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.x = cpu.a;
                cpu.set(Flag::Negative, cpu.x & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.x == 0));
                true
            }
            Instruction::TAY => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.y = cpu.a;
                cpu.set(Flag::Negative, cpu.y & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.y == 0));
                true
            }
            Instruction::TXA => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.a = cpu.x;
                cpu.set(Flag::Negative, cpu.a & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.a == 0));
                true
            }
            Instruction::TYA => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.a = cpu.y;
                cpu.set(Flag::Negative, cpu.a & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.a == 0));
                true
            }
            Instruction::TSX => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.x = cpu.sp;
                cpu.set(Flag::Negative, cpu.x & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.x == 0));
                true
            }
            Instruction::TXS => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.sp = cpu.x;
                true
            }
            Instruction::PHA => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.push_stack(cpu.a);
                true
            }
            Instruction::PHP => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.push_stack(cpu.ps);
                true
            }
            Instruction::PLA => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.a = cpu.pop_stack();
                cpu.set(Flag::Negative, cpu.a & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.a == 0));
                true
            }
            Instruction::PLP => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.ps = cpu.pop_stack();
                true
            }

            Instruction::AND => logical::and(arg, cpu),
            Instruction::EOR => logical::eor(arg, cpu),
            Instruction::ORA => logical::ora(arg, cpu),
            Instruction::BIT => logical::bit(arg, cpu),

            Instruction::ADC => arithmetic::adc(arg, cpu),
            Instruction::SBC => arithmetic::sbc(arg, cpu),
            Instruction::CMP => arithmetic::cmp(arg, cpu),
            Instruction::CPX => arithmetic::cpx(arg, cpu),
            Instruction::CPY => arithmetic::cpy(arg, cpu),

            Instruction::INC => increment_decrement::inc(arg, cpu),
            Instruction::INX => increment_decrement::inx(arg, cpu),
            Instruction::INY => increment_decrement::iny(arg, cpu),
            Instruction::DEC => increment_decrement::dec(arg, cpu),
            Instruction::DEX => increment_decrement::dex(arg, cpu),
            Instruction::DEY => increment_decrement::dey(arg, cpu),

            Instruction::ASL => shift::asl(arg, cpu),
            Instruction::LSR => shift::lsr(arg, cpu),
            Instruction::ROL => shift::rol(arg, cpu),
            Instruction::ROR => shift::ror(arg, cpu),

            // Jumps & Calls
            Instruction::JMP => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                cpu.pc = addr;

                false
            }
            Instruction::JSR => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                cpu.push_stack(Byte::from(cpu.pc >> 8));
                cpu.push_stack(Byte::from((cpu.pc & 0x00ff) + 3));
                cpu.pc = addr;

                false
            }
            Instruction::RTS => {
                assert!(matches!(arg, InstructionArgument::Implied));
                let low_addr = cpu.pop_stack();
                let hi_addr = cpu.pop_stack();
                cpu.pc = (Addr::from(hi_addr) << 8) | low_addr;

                false
            }

            // Branches
            Instruction::BCC => {
                let InstructionArgument::Offset(offset) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };

                if !cpu.is_set(Flag::Carry) {
                    if (offset & Flag::Negative).0 {
                        cpu.pc = cpu.pc - (!*offset + 1) + 2;
                    } else {
                        cpu.pc = cpu.pc + offset + 2;
                    }
                    return false;
                }
                true
            }
            Instruction::BCS => {
                let InstructionArgument::Offset(offset) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if cpu.is_set(Flag::Carry) {
                    if (offset & Flag::Negative).0 {
                        cpu.pc = cpu.pc - (!*offset + 1) + 2;
                    } else {
                        cpu.pc = cpu.pc + offset + 2;
                    }
                    return false;
                }
                true
            }
            Instruction::BEQ => {
                let InstructionArgument::Offset(offset) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if cpu.is_set(Flag::Zero) {
                    if (offset & Flag::Negative).0 {
                        cpu.pc = cpu.pc - (!*offset + 1) + 2;
                    } else {
                        cpu.pc = cpu.pc + offset + 2;
                    }
                    return false;
                }
                true
            }
            Instruction::BMI => {
                let InstructionArgument::Offset(offset) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if cpu.is_set(Flag::Negative) {
                    if (offset & Flag::Negative).0 {
                        cpu.pc = cpu.pc - (!*offset + 1) + 2;
                    } else {
                        cpu.pc = cpu.pc + offset + 2;
                    }
                    return false;
                }
                true
            }
            Instruction::BNE => {
                let InstructionArgument::Offset(offset) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if !cpu.is_set(Flag::Zero) {
                    if (offset & Flag::Negative).0 {
                        cpu.pc = cpu.pc - (!*offset + 1) + 2;
                    } else {
                        cpu.pc = cpu.pc + offset + 2;
                    }
                    return false;
                }
                true
            }
            Instruction::BPL => {
                let InstructionArgument::Offset(offset) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if !cpu.is_set(Flag::Negative) {
                    if (offset & Flag::Negative).0 {
                        cpu.pc = cpu.pc - (!*offset + 1) + 2;
                    } else {
                        cpu.pc = cpu.pc + offset + 2;
                    }
                    return false;
                }
                true
            }
            Instruction::BVC => {
                let InstructionArgument::Offset(offset) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if !cpu.is_set(Flag::Overflow) {
                    if (offset & Flag::Negative).0 {
                        cpu.pc = cpu.pc - (!*offset + 1) + 2;
                    } else {
                        cpu.pc = cpu.pc + offset + 2;
                    }
                    return false;
                }
                true
            }
            Instruction::BVS => {
                let InstructionArgument::Offset(offset) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if cpu.is_set(Flag::Overflow) {
                    if (offset & Flag::Negative).0 {
                        cpu.pc = cpu.pc - (!*offset + 1) + 2;
                    } else {
                        cpu.pc = cpu.pc + offset + 2;
                    }
                    return false;
                }
                true
            }

            // Status Flag Changes
            Instruction::CLC => {
                cpu.set(Flag::Carry, Bit(false));
                true
            }
            Instruction::CLD => {
                cpu.set(Flag::DecimalMode, Bit(false));
                true
            }
            Instruction::CLI => {
                cpu.set(Flag::InterruptDisable, Bit(false));
                true
            }
            Instruction::CLV => {
                cpu.set(Flag::Overflow, Bit(false));
                true
            }
            Instruction::SEC => {
                cpu.set(Flag::Carry, Bit(true));
                true
            }
            Instruction::SED => {
                cpu.set(Flag::DecimalMode, Bit(true));
                true
            }
            Instruction::SEI => {
                cpu.set(Flag::InterruptDisable, Bit(true));
                true
            }

            // System Functions
            Instruction::BRK => {
                cpu.push_stack(Byte::from(cpu.pc >> 8));
                cpu.push_stack(Byte::from(cpu.pc & 0xff));
                cpu.push_stack(cpu.ps);
                let low_addr = cpu.read(Addr::from(0xfffe));
                let hi_addr = cpu.read(Addr::from(0xffff));
                cpu.pc = (Addr::from(hi_addr) << 8) | low_addr;
                cpu.set(Flag::BreakCmd, Bit(true));
                false
            }
            Instruction::NOP => true,
            Instruction::RTI => {
                cpu.ps = cpu.pop_stack();
                cpu.set(Flag::BreakCmd, Bit(false));
                let hi_addr = cpu.pop_stack();
                let low_addr = cpu.pop_stack();
                cpu.pc = (Addr::from(hi_addr) << 8) | low_addr;
                cpu.unpend_irq();
                false
            }
            Instruction::XXX => {
                cpu.halt();
                false
            }
        }
    }

    pub fn valid_address_mode(&self, addressing_mode: AddressingMode) -> bool {
        match self {
            Instruction::ADC => matches!(
                addressing_mode,
                AddressingMode::Immediate
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
            ),

            Instruction::AND => matches!(
                addressing_mode,
                AddressingMode::Immediate
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::Indirect
            ),

            Instruction::ASL => matches!(
                addressing_mode,
                AddressingMode::Implied
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
            ),

            Instruction::TSX => matches!(addressing_mode, AddressingMode::Implied),

            Instruction::CMP => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
            ),

            Instruction::RTS => matches!(addressing_mode, AddressingMode::Implied),

            Instruction::DEX => matches!(addressing_mode, AddressingMode::Implied),

            Instruction::DEY => matches!(addressing_mode, AddressingMode::Implied),

            Instruction::STA => matches!(
                addressing_mode,
                AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
            ),

            Instruction::PHA => matches!(addressing_mode, AddressingMode::Implied),

            Instruction::LDA => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
            ),

            Instruction::LSR => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
            ),

            Instruction::ORA => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
            ),

            Instruction::SEC => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::SED => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::BNE => matches!(addressing_mode, AddressingMode::Relative),
            Instruction::PLA => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::PHP => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::SEI => matches!(addressing_mode, AddressingMode::Implied),

            Instruction::STX => matches!(
                addressing_mode,
                AddressingMode::ZeroPageY | AddressingMode::Absolute
            ),

            Instruction::STY => matches!(
                addressing_mode,
                AddressingMode::ZeroPageX | AddressingMode::Absolute
            ),

            Instruction::LDX => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageY
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteY
            ),

            Instruction::LDY => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
            ),

            Instruction::ROL => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
            ),

            Instruction::BPL => matches!(addressing_mode, AddressingMode::Relative),
            Instruction::PLP => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::CLC => matches!(addressing_mode, AddressingMode::Implied),

            Instruction::ROR => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
            ),

            Instruction::CLD => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::BRK => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::BCC => matches!(addressing_mode, AddressingMode::Relative),
            Instruction::CLI => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::BVC => matches!(addressing_mode, AddressingMode::Relative),
            Instruction::TXA => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::TAX => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::NOP => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::TAY => matches!(addressing_mode, AddressingMode::Implied),

            Instruction::EOR => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
            ),

            Instruction::CLV => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::BCS => matches!(addressing_mode, AddressingMode::Relative),
            Instruction::JMP => matches!(addressing_mode, AddressingMode::Absolute),

            Instruction::BVS => matches!(addressing_mode, AddressingMode::Relative),
            Instruction::BEQ => matches!(addressing_mode, AddressingMode::Relative),
            Instruction::TXS => matches!(addressing_mode, AddressingMode::Implied),

            Instruction::SBC => matches!(
                addressing_mode,
                AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX
                    | AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
            ),

            Instruction::CPX => matches!(
                addressing_mode,
                AddressingMode::ZeroPage | AddressingMode::Absolute
            ),

            Instruction::CPY => matches!(
                addressing_mode,
                AddressingMode::ZeroPage | AddressingMode::Absolute
            ),

            Instruction::BIT => matches!(addressing_mode, AddressingMode::Absolute),

            Instruction::BMI => matches!(addressing_mode, AddressingMode::Relative),

            Instruction::JSR => matches!(addressing_mode, AddressingMode::Absolute),

            Instruction::INC => matches!(
                addressing_mode,
                AddressingMode::ZeroPageX | AddressingMode::Absolute | AddressingMode::AbsoluteX
            ),

            Instruction::DEC => matches!(
                addressing_mode,
                AddressingMode::ZeroPageX | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY
            ),

            Instruction::INX => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::INY => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::RTI => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::TYA => matches!(addressing_mode, AddressingMode::Implied),
            Instruction::XXX => matches!(addressing_mode, AddressingMode::Implied),
        }
    }
}

pub fn get_instruction(op_code: Byte) -> (Instruction, AddressingMode) {
    if INSTRUCTIONS.contains_key(&op_code) {
        INSTRUCTIONS[&op_code.0]
    } else {
        (Instruction::NOP, AddressingMode::Implied)
    }
}

pub static INSTRUCTIONS: phf::Map<u8, (Instruction, AddressingMode)> = phf_map! {
    0x69u8 => (Instruction::ADC, AddressingMode::Immediate),
    0x65u8 => (Instruction::ADC, AddressingMode::ZeroPage),
    0x75u8 => (Instruction::ADC, AddressingMode::ZeroPageX),
    0x6Du8 => (Instruction::ADC, AddressingMode::Absolute),
    0x7Du8 => (Instruction::ADC, AddressingMode::AbsoluteX),
    0x79u8 => (Instruction::ADC, AddressingMode::AbsoluteY),
    0x61u8 => (Instruction::ADC, AddressingMode::IndirectX),
    0x71u8 => (Instruction::ADC, AddressingMode::IndirectY),

    0x29u8 => (Instruction::AND, AddressingMode::Immediate),
    0x25u8 => (Instruction::AND, AddressingMode::ZeroPage),
    0x35u8 => (Instruction::AND, AddressingMode::ZeroPageX),
    0x2Du8 => (Instruction::AND, AddressingMode::Absolute),
    0x3Du8 => (Instruction::AND, AddressingMode::AbsoluteX),
    0x39u8 => (Instruction::AND, AddressingMode::AbsoluteY),
    0x21u8 => (Instruction::AND, AddressingMode::IndirectX),
    0x31u8 => (Instruction::AND, AddressingMode::Indirect),

    0x0Au8 => (Instruction::ASL, AddressingMode::Implied),
    0x06u8 => (Instruction::ASL, AddressingMode::ZeroPage),
    0x16u8 => (Instruction::ASL, AddressingMode::ZeroPageX),
    0x0Eu8 => (Instruction::ASL, AddressingMode::Absolute),
    0x1Eu8 => (Instruction::ASL, AddressingMode::AbsoluteX),

    0xBAu8 => (Instruction::TSX, AddressingMode::Implied),

    0xC9u8 => (Instruction::CMP, AddressingMode::Immediate),
    0xC5u8 => (Instruction::CMP, AddressingMode::ZeroPage),
    0xD5u8 => (Instruction::CMP, AddressingMode::ZeroPageX),
    0xCDu8 => (Instruction::CMP, AddressingMode::Absolute),
    0xDDu8 => (Instruction::CMP, AddressingMode::AbsoluteX),
    0xD9u8 => (Instruction::CMP, AddressingMode::AbsoluteY),
    0xC1u8 => (Instruction::CMP, AddressingMode::IndirectX),
    0xD1u8 => (Instruction::CMP, AddressingMode::IndirectY),

    0x60u8 => (Instruction::RTS, AddressingMode::Implied),

    0xCAu8 => (Instruction::DEX, AddressingMode::Implied),

    0x88u8 => (Instruction::DEY, AddressingMode::Implied),

    0x85u8 => (Instruction::STA, AddressingMode::ZeroPage),
    0x95u8 => (Instruction::STA, AddressingMode::ZeroPageX),
    0x8Du8 => (Instruction::STA, AddressingMode::Absolute),
    0x9Du8 => (Instruction::STA, AddressingMode::AbsoluteX),
    0x99u8 => (Instruction::STA, AddressingMode::AbsoluteY),
    0x81u8 => (Instruction::STA, AddressingMode::IndirectX),
    0x91u8 => (Instruction::STA, AddressingMode::IndirectY),

    0x48u8 => (Instruction::PHA, AddressingMode::Implied),

    0xA9u8 => (Instruction::LDA, AddressingMode::Immediate),
    0xA5u8 => (Instruction::LDA, AddressingMode::ZeroPage),
    0xB5u8 => (Instruction::LDA, AddressingMode::ZeroPageX),
    0xADu8 => (Instruction::LDA, AddressingMode::Absolute),
    0xBDu8 => (Instruction::LDA, AddressingMode::AbsoluteX),
    0xB9u8 => (Instruction::LDA, AddressingMode::AbsoluteY),
    0xA1u8 => (Instruction::LDA, AddressingMode::IndirectX),
    0xB1u8 => (Instruction::LDA, AddressingMode::IndirectY),

    0x4Au8 => (Instruction::LSR, AddressingMode::Implied),
    0x46u8 => (Instruction::LSR, AddressingMode::ZeroPage),
    0x56u8 => (Instruction::LSR, AddressingMode::ZeroPageX),
    0x4Eu8 => (Instruction::LSR, AddressingMode::Absolute),
    0x5Eu8 => (Instruction::LSR, AddressingMode::AbsoluteX),

    0x09u8 => (Instruction::ORA, AddressingMode::Immediate),
    0x05u8 => (Instruction::ORA, AddressingMode::ZeroPage),
    0x15u8 => (Instruction::ORA, AddressingMode::ZeroPageX),
    0x0Du8 => (Instruction::ORA, AddressingMode::Absolute),
    0x1Du8 => (Instruction::ORA, AddressingMode::AbsoluteX),
    0x19u8 => (Instruction::ORA, AddressingMode::AbsoluteY),
    0x01u8 => (Instruction::ORA, AddressingMode::IndirectX),
    0x11u8 => (Instruction::ORA, AddressingMode::IndirectY),

    0x38u8 => (Instruction::SEC, AddressingMode::Implied),
    0xF8u8 => (Instruction::SED, AddressingMode::Implied),
    0xD0u8 => (Instruction::BNE, AddressingMode::Relative),
    0x68u8 => (Instruction::PLA, AddressingMode::Implied),
    0x08u8 => (Instruction::PHP, AddressingMode::Implied),
    0x78u8 => (Instruction::SEI, AddressingMode::Implied),

    0x86u8 => (Instruction::STX, AddressingMode::ZeroPage),
    0x96u8 => (Instruction::STX, AddressingMode::ZeroPageY),
    0x8Eu8 => (Instruction::STX, AddressingMode::Absolute),

    0x84u8 => (Instruction::STY, AddressingMode::ZeroPage),
    0x94u8 => (Instruction::STY, AddressingMode::ZeroPageX),
    0x8Cu8 => (Instruction::STY, AddressingMode::Absolute),

    0xA2u8 => (Instruction::LDX, AddressingMode::Immediate),
    0xA6u8 => (Instruction::LDX, AddressingMode::ZeroPage),
    0xB6u8 => (Instruction::LDX, AddressingMode::ZeroPageY),
    0xAEu8 => (Instruction::LDX, AddressingMode::Absolute),
    0xBEu8 => (Instruction::LDX, AddressingMode::AbsoluteY),

    0xA0u8 => (Instruction::LDY, AddressingMode::Immediate),
    0xA4u8 => (Instruction::LDY, AddressingMode::ZeroPage),
    0xB4u8 => (Instruction::LDY, AddressingMode::ZeroPageX),
    0xACu8 => (Instruction::LDY, AddressingMode::Absolute),
    0xBCu8 => (Instruction::LDY, AddressingMode::AbsoluteX),

    0x2Au8 => (Instruction::ROL, AddressingMode::Implied),
    0x26u8 => (Instruction::ROL, AddressingMode::ZeroPage),
    0x36u8 => (Instruction::ROL, AddressingMode::ZeroPageX),
    0x2Eu8 => (Instruction::ROL, AddressingMode::Absolute),
    0x3Eu8 => (Instruction::ROL, AddressingMode::AbsoluteX),

    0x10u8 => (Instruction::BPL, AddressingMode::Relative),
    0x28u8 => (Instruction::PLP, AddressingMode::Implied),
    0x18u8 => (Instruction::CLC, AddressingMode::Implied),

    0x6Au8 => (Instruction::ROR, AddressingMode::Implied),
    0x66u8 => (Instruction::ROR, AddressingMode::ZeroPage),
    0x76u8 => (Instruction::ROR, AddressingMode::ZeroPageX),
    0x6Eu8 => (Instruction::ROR, AddressingMode::Absolute),
    0x7Eu8 => (Instruction::ROR, AddressingMode::AbsoluteX),

    0xD8u8 => (Instruction::CLD, AddressingMode::Implied),
    0x00u8 => (Instruction::BRK, AddressingMode::Implied),
    0x90u8 => (Instruction::BCC, AddressingMode::Relative),
    0x58u8 => (Instruction::CLI, AddressingMode::Implied),
    0x50u8 => (Instruction::BVC, AddressingMode::Relative),
    0x8Au8 => (Instruction::TXA, AddressingMode::Implied),
    0xAAu8 => (Instruction::TAX, AddressingMode::Implied),
    0xEAu8 => (Instruction::NOP, AddressingMode::Implied),
    0xA8u8 => (Instruction::TAY, AddressingMode::Implied),

    0x49u8 => (Instruction::EOR, AddressingMode::Immediate),
    0x45u8 => (Instruction::EOR, AddressingMode::ZeroPage),
    0x55u8 => (Instruction::EOR, AddressingMode::ZeroPageX),
    0x4Du8 => (Instruction::EOR, AddressingMode::Absolute),
    0x5Du8 => (Instruction::EOR, AddressingMode::AbsoluteX),
    0x59u8 => (Instruction::EOR, AddressingMode::AbsoluteY),
    0x41u8 => (Instruction::EOR, AddressingMode::IndirectX),
    0x51u8 => (Instruction::EOR, AddressingMode::IndirectY),

    0xB8u8 => (Instruction::CLV, AddressingMode::Implied),
    0xB0u8 => (Instruction::BCS, AddressingMode::Relative),
    0x4Cu8 => (Instruction::JMP, AddressingMode::Absolute),
    0x6Cu8 => (Instruction::JMP, AddressingMode::Indirect),

    0x70u8 => (Instruction::BVS, AddressingMode::Relative),
    0xF0u8 => (Instruction::BEQ, AddressingMode::Relative),
    0x9Au8 => (Instruction::TXS, AddressingMode::Implied),

    0xE9u8 => (Instruction::SBC, AddressingMode::Immediate),
    0xE5u8 => (Instruction::SBC, AddressingMode::ZeroPage),
    0xF5u8 => (Instruction::SBC, AddressingMode::ZeroPageX),
    0xEDu8 => (Instruction::SBC, AddressingMode::Absolute),
    0xFDu8 => (Instruction::SBC, AddressingMode::AbsoluteX),
    0xF9u8 => (Instruction::SBC, AddressingMode::AbsoluteY),
    0xE1u8 => (Instruction::SBC, AddressingMode::IndirectX),
    0xF1u8 => (Instruction::SBC, AddressingMode::IndirectY),

    0xE0u8 => (Instruction::CPX, AddressingMode::Immediate),
    0xE4u8 => (Instruction::CPX, AddressingMode::ZeroPage),
    0xECu8 => (Instruction::CPX, AddressingMode::Absolute),

    0xC0u8 => (Instruction::CPY, AddressingMode::Immediate),
    0xC4u8 => (Instruction::CPY, AddressingMode::ZeroPage),
    0xCCu8 => (Instruction::CPY, AddressingMode::Absolute),

    0x24u8 => (Instruction::BIT, AddressingMode::ZeroPage),
    0x2Cu8 => (Instruction::BIT, AddressingMode::Absolute),

    0x30u8 => (Instruction::BMI, AddressingMode::Relative),

    0x20u8 => (Instruction::JSR, AddressingMode::Absolute),

    0xE6u8 => (Instruction::INC, AddressingMode::ZeroPage),
    0xF6u8 => (Instruction::INC, AddressingMode::ZeroPageX),
    0xEEu8 => (Instruction::INC, AddressingMode::Absolute),
    0xFEu8 => (Instruction::INC, AddressingMode::AbsoluteX),

    0xC6u8 => (Instruction::DEC, AddressingMode::ZeroPage),
    0xD6u8 => (Instruction::DEC, AddressingMode::ZeroPageX),
    0xCEu8 => (Instruction::DEC, AddressingMode::Absolute),
    0xDEu8 => (Instruction::DEC, AddressingMode::AbsoluteX),

    0xE8u8 => (Instruction::INX, AddressingMode::Implied),
    0xC8u8 => (Instruction::INY, AddressingMode::Implied),
    0x40u8 => (Instruction::RTI, AddressingMode::Implied),
    0x98u8 => (Instruction::TYA, AddressingMode::Implied),
    0xffu8 => (Instruction::XXX, AddressingMode::Implied),
};
