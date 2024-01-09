pub mod arithmetic;
pub mod increment_decrement;
pub mod load_store;
pub mod logical;
pub mod shift;

use std::fmt::Display;

use crate::hardware::cpu::{Flag, CPU};
use crate::types::*;

#[derive(Debug)]
pub enum InstructionArgument {
    Immediate(Byte),
    Offset(Byte),
    Address(Addr),
    Implied,
}

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
    pub fn get(self, cpu: &mut CPU) -> InstructionArgument {
        match self {
            AddressingMode::Immediate => InstructionArgument::Immediate(cpu.next_instruction()),
            AddressingMode::ZeroPage => {
                InstructionArgument::Address(Addr::from(cpu.next_instruction()))
            }
            AddressingMode::ZeroPageX => {
                InstructionArgument::Address(Addr::from(cpu.next_instruction() + cpu.x))
            }
            AddressingMode::ZeroPageY => {
                InstructionArgument::Address(Addr::from(cpu.next_instruction() + cpu.y))
            }
            AddressingMode::Absolute => {
                let low_addr = cpu.next_instruction();
                let hi_addr = cpu.next_instruction();
                InstructionArgument::Address(Addr::from(hi_addr) << 8 | low_addr)
            }
            AddressingMode::AbsoluteX => {
                let low_addr = cpu.next_instruction();
                let hi_addr = cpu.next_instruction();
                InstructionArgument::Address(((Addr::from(hi_addr) << 8) | low_addr) + cpu.x)
            }
            AddressingMode::AbsoluteY => {
                let low_addr = cpu.next_instruction();
                let hi_addr = cpu.next_instruction();
                InstructionArgument::Address(((Addr::from(hi_addr) << 8) | low_addr) + cpu.y)
            }
            AddressingMode::Indirect => {
                let low_addr = cpu.next_instruction();
                let hi_addr = cpu.next_instruction();
                let _addr = ((Addr::from(hi_addr) << 8) | low_addr) + cpu.y;
                InstructionArgument::Address(
                    Addr::from((cpu.read_memory(_addr)) << 8) | cpu.read_memory(_addr + 1),
                )
            }
            AddressingMode::IndirectX => {
                let _addr = cpu.next_instruction() + cpu.x;
                let low_addr = cpu.read_memory(_addr.into());
                let hi_addr = cpu.read_memory((_addr + 1).into());
                InstructionArgument::Address(Addr::from(hi_addr << 8) | low_addr)
            }
            AddressingMode::IndirectY => {
                let _addr = cpu.next_instruction();
                let low_addr = cpu.read_memory(_addr.into());
                let hi_addr = cpu.read_memory((_addr + 1).into());
                InstructionArgument::Address(((Addr::from(hi_addr) << 8) | low_addr) + cpu.y)
            }
            AddressingMode::Relative => {
                InstructionArgument::Offset(cpu.next_instruction())
            }
            AddressingMode::Implied => InstructionArgument::Implied,
        }
    }
}

#[derive(Debug)]
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

impl Instruction {
    pub fn exec(&self, arg: InstructionArgument, cpu: &mut CPU) {
        match self {
            Instruction::LDA => load_store::lda(arg, cpu),
            Instruction::LDX => load_store::ldx(arg, cpu),
            Instruction::LDY => load_store::ldy(arg, cpu),
            Instruction::STA => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                cpu.write_memory(addr, cpu.a)
            }
            Instruction::STX => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                cpu.write_memory(addr, cpu.x);
            }
            Instruction::STY => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                cpu.write_memory(addr, cpu.y);
            }
            Instruction::TAX => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.x = cpu.a;
                cpu.set(Flag::Negative, cpu.x & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.x == 0));
            }
            Instruction::TAY => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.y = cpu.a;
                cpu.set(Flag::Negative, cpu.y & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.y == 0));
            }
            Instruction::TXA => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.a = cpu.x;
                cpu.set(Flag::Negative, cpu.a & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.a == 0));
            }
            Instruction::TYA => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.a = cpu.y;
                cpu.set(Flag::Negative, cpu.a & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.a == 0));
            }
            Instruction::TSX => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.x = cpu.sp;
                cpu.set(Flag::Negative, cpu.x & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.x == 0));
            }
            Instruction::TXS => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.sp = cpu.x;
            }
            Instruction::PHA => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.push_stack(cpu.a);
            }
            Instruction::PHP => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.push_stack(cpu.ps);
            }
            Instruction::PLA => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.a = cpu.pop_stack();
                cpu.set(Flag::Negative, cpu.a & Flag::Negative);
                cpu.set(Flag::Zero, Bit(cpu.a == 0));
            }
            Instruction::PLP => {
                assert!(matches!(arg, InstructionArgument::Implied));
                cpu.ps = cpu.pop_stack();
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
            }
            Instruction::JSR => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                cpu.push_stack(Byte::from(cpu.pc >> 8));
                cpu.push_stack(Byte::from((cpu.pc & 0xff) + 1));
                cpu.pc = addr;
            }
            Instruction::RTS => {
                assert!(matches!(arg, InstructionArgument::Implied));
                let low_addr = cpu.pop_stack();
                let hi_addr = cpu.pop_stack();
                cpu.pc = (Addr::from(hi_addr) << 8) | low_addr;
            }

            // Branches
            Instruction::BCC => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };

                if !cpu.is_set(Flag::Carry) {
                    cpu.pc = addr;
                }
            }
            Instruction::BCS => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if cpu.is_set(Flag::Carry) {
                    cpu.pc = addr;
                }
            }
            Instruction::BEQ => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if cpu.is_set(Flag::Zero) {
                    cpu.pc = addr;
                }
            }
            Instruction::BMI => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if cpu.is_set(Flag::Negative) {
                    cpu.pc = addr;
                }
            }
            Instruction::BNE => {
                let InstructionArgument::Offset(offset) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if !cpu.is_set(Flag::Zero) {
                    if (offset & Flag::Negative).0 {
                        cpu.pc = cpu.pc - (!*offset + 1);
                    }
                }
            }
            Instruction::BPL => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if !cpu.is_set(Flag::Negative) {
                    cpu.pc += *addr;
                }
            }
            Instruction::BVC => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if !cpu.is_set(Flag::Overflow) {
                    cpu.pc += *addr;
                }
            }
            Instruction::BVS => {
                let InstructionArgument::Address(addr) = arg else {
                    unreachable!("Illegal addressing mode: {:?}", arg);
                };
                if cpu.is_set(Flag::Overflow) {
                    cpu.pc = addr;
                }
            }

            // Status Flag Changes
            Instruction::CLC => cpu.set(Flag::Carry, Bit(false)),
            Instruction::CLD => cpu.set(Flag::DecimalMode, Bit(false)),
            Instruction::CLI => cpu.set(Flag::InterruptDisable, Bit(false)),
            Instruction::CLV => cpu.set(Flag::Overflow, Bit(false)),
            Instruction::SEC => cpu.set(Flag::Carry, Bit(true)),
            Instruction::SED => cpu.set(Flag::DecimalMode, Bit(true)),
            Instruction::SEI => cpu.set(Flag::InterruptDisable, Bit(true)),

            // System Functions
            Instruction::BRK => {
                cpu.push_stack(Byte::from(cpu.pc >> 8));
                cpu.push_stack(Byte::from(cpu.pc & 0xff));
                cpu.push_stack(cpu.ps);
                let low_addr = cpu.read_memory(Addr::from(0xfffe));
                let hi_addr = cpu.read_memory(Addr::from(0xffff));
                cpu.pc = (Addr::from(hi_addr) << 8) | low_addr;
                cpu.set(Flag::BreakCmd, Bit(true));
            }
            Instruction::NOP => {}
            Instruction::RTI => {
                cpu.ps = cpu.pop_stack();
                cpu.set(Flag::BreakCmd, Bit(false));
                let low_addr = cpu.pop_stack();
                let hi_addr = cpu.pop_stack();
                cpu.pc = (Addr::from(hi_addr) << 8) | low_addr;
            }
            Instruction::XXX => cpu.halt(),
        }
    }
}

#[rustfmt::skip]
impl From<Byte> for AddressingMode {
    fn from(value: Byte) -> Self {
        match value.0 {
            0x69 | 0x29 | 0xe0 | 0xc0 | 0x49 | 0xa9 | 0xa2 | 0xa0 | 0x09 | 0xe9 => AddressingMode::Immediate,
            0x65 | 0x25 | 0x06 | 0x24 | 0xc5 | 0xe4 | 0xc4 | 0xc6 | 0x45 | 0xe6 | 0xa5 | 0xa6
            | 0xa4 | 0x46 | 0x05 | 0x26 | 0x66 | 0xe5 | 0x85 | 0x86 | 0x84 => AddressingMode::ZeroPage,
            0x75 | 0x35 | 0x16 | 0xd5 | 0xd6 | 0x55 | 0xf6 | 0xb5 | 0xb4 | 0x56 | 0x15 | 0x36
            | 0x76 | 0xf5 | 0x95 | 0x94 => AddressingMode::ZeroPageX,
            0xb6 | 0x96 => AddressingMode::ZeroPageY,
            0x6d | 0x2d | 0x0e | 0x2c | 0xcd | 0xec | 0xcc | 0xce | 0x4d | 0xee | 0x4c | 0x20
            | 0xad | 0xae | 0xac | 0x4e | 0x0d | 0x2e | 0x6e | 0xed | 0x8d | 0x8e | 0x8c => AddressingMode::Absolute,
            0x7d | 0x3d | 0x1e | 0xdd | 0xde | 0x5d | 0xfe | 0xbd | 0xbc | 0x5e | 0x1d | 0x3e
            | 0x7e | 0xfd | 0x9d => AddressingMode::AbsoluteX,
            0x79 | 0x39 | 0xd9 | 0x59 | 0xb9 | 0xbe | 0x19 | 0xf9 | 0x99 => AddressingMode::AbsoluteY,
            0x6c => AddressingMode::Indirect,
            0x61 | 0x21 | 0xc1 | 0x41 | 0xa1 | 0x01 | 0xe1 | 0x81 => AddressingMode::IndirectX,
            0x71 | 0x31 | 0xd1 | 0x51 | 0xb1 | 0x11 | 0xf1 | 0x91 => AddressingMode::IndirectY,
            0x90 | 0xb0 | 0xf0 | 0x30 | 0xd0 | 0x10 | 0x50 | 0x70 => AddressingMode::Relative,
            0x00 | 0x18 | 0xd8 | 0x58 | 0xb8 | 0xca | 0x88 | 0xe8 | 0xc8 | 0xea | 0x48 | 0x08
            | 0x68 | 0x28 | 0x40 | 0x60 | 0x38 | 0xf8 | 0x78 | 0xaa | 0xa8 | 0xba | 0x8a | 0x9a
            | 0x0a | 0x4a | 0x2a | 0x7a => AddressingMode::Implied,
            0xff => AddressingMode::Implied,
            _ => unreachable!("Illegal opcode: {}", value.0)
        }
    }
}

impl From<Byte> for Instruction {
    fn from(value: Byte) -> Self {
        match value.0 {
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => Instruction::ADC,
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => Instruction::AND,
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => Instruction::ASL,
            0xBA => Instruction::TSX,
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => Instruction::CMP,
            0x60 => Instruction::RTS,
            0xCA => Instruction::DEX,
            0x88 => Instruction::DEY,
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => Instruction::STA,
            0x48 => Instruction::PHA,
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => Instruction::LDA,
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => Instruction::LSR,
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => Instruction::ORA,
            0x38 => Instruction::SEC,
            0xF8 => Instruction::SED,
            0xD0 => Instruction::BNE,
            0x68 => Instruction::PLA,
            0x08 => Instruction::PHP,
            0x78 => Instruction::SEI,
            0x86 | 0x96 | 0x8E => Instruction::STX,
            0x84 | 0x94 | 0x8C => Instruction::STY,
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => Instruction::LDX,
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => Instruction::LDY,
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => Instruction::ROL,
            0x10 => Instruction::BPL,
            0x28 => Instruction::PLP,
            0x18 => Instruction::CLC,
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => Instruction::ROR,
            0xD8 => Instruction::CLD,
            0x00 => Instruction::BRK,
            0x90 => Instruction::BCC,
            0x58 => Instruction::CLI,
            0x50 => Instruction::BVC,
            0x8A => Instruction::TXA,
            0xAA => Instruction::TAX,
            0xEA => Instruction::NOP,
            0xA8 => Instruction::TAY,
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => Instruction::EOR,
            0xB8 => Instruction::CLV,
            0xB0 => Instruction::BCS,
            0x4C | 0x6C => Instruction::JMP,
            0x70 => Instruction::BVS,
            0xF0 => Instruction::BEQ,
            0x9A => Instruction::TXS,
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => Instruction::SBC,
            0xE0 | 0xE4 | 0xEC => Instruction::CPX,
            0xC0 | 0xC4 | 0xCC => Instruction::CPY,
            0x24 | 0x2C => Instruction::BIT,
            0x30 => Instruction::BMI,
            0x20 => Instruction::JSR,
            0xE6 | 0xF6 | 0xEE | 0xFE => Instruction::INC,
            0xC6 | 0xD6 | 0xCE | 0xDE => Instruction::DEC,
            0xE8 => Instruction::INX,
            0xC8 => Instruction::INY,
            0x40 => Instruction::RTI,
            0x98 => Instruction::TYA,
            0xff => Instruction::XXX,
            _ => unreachable!("Illegal opcode: {}", value.0),
        }
    }
}
