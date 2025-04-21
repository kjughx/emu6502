use either::Either;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::sync::atomic::AtomicBool;

use crate::types::*;

use crate::hardware::cpu::{Register, CPU};

pub struct Debugger {
    cpu: CPU,
    cmd: Option<Command>,
}

pub static RUNNING: AtomicBool = const { AtomicBool::new(false) };

impl Debugger {
    pub fn new(mut cpu: CPU, reset: bool) -> Self {
        cpu.debug();
        if reset {
            cpu.reset();
        }
        Self { cpu, cmd: None }
    }

    pub fn start(mut self) -> ! {
        let mut rl = rustyline::DefaultEditor::new().unwrap();
        loop {
            if RUNNING.load(std::sync::atomic::Ordering::Acquire) {
                if !self.cpu.debug_exec() {
                    RUNNING.store(false, std::sync::atomic::Ordering::Release)
                }
                continue;
            }

            let cmd = match Command::get_cmd(&mut rl) {
                Err(e) => {
                    println!("ERROR: {e}");
                    continue;
                }
                Ok(cmd) => cmd,
            };

            if matches!(cmd, Command::Nothing) {
                if let Some(cmd) = self.cmd {
                    self.handle_cmd(cmd);
                }
                continue;
            }

            self.handle_cmd(cmd);
            self.cmd = Some(cmd);
        }
    }

    fn handle_cmd(&mut self, cmd: Command) {
        use Command::*;
        match cmd {
            Break(addr) => self.cpu.breakpoint(addr),
            Run => RUNNING.store(true, std::sync::atomic::Ordering::Release),
            Step => {
                self.cpu.debug_exec();
            }
            StoreMem(addr, val) => self.cpu.write(addr, val),
            StoreReg(reg, val) => self.cpu.set_reg(reg, Either::Left(val)),
            StoreRegLong(reg, val) => self.cpu.set_reg(reg, Either::Right(val)),
            LoadMem(addr) => {
                let val = self.cpu.read(*addr);
                println!("{val}");
            }
            LoadReg(reg) => {
                let val = self.cpu.get_reg(reg);
                println!("{val}");
            }
            Help => {
                println!("{USAGE}\n");
            }
            ShowRegs => {
                println!(
                    "A:  {}\nX:  {}\nY:  {}\nPS: 0b{:b}\nSP: {}\nPC: {}\n",
                    self.cpu.get_reg(Register::A).unwrap_left(),
                    self.cpu.get_reg(Register::X).unwrap_left(),
                    self.cpu.get_reg(Register::Y).unwrap_left(),
                    self.cpu.get_reg(Register::PS).unwrap_left().0,
                    self.cpu.get_reg(Register::SP).unwrap_left(),
                    self.cpu.get_reg(Register::PC).unwrap_right(),
                )
            }
            Nothing => (),
        }
    }
}

impl TryFrom<&str> for Register {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "a" => Ok(Register::A),
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "ps" => Ok(Register::PS),
            "pc" => Ok(Register::PC),
            "sp" => Ok(Register::SP),
            _ => Err(format!("Invalid register: {value}")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Help,
    Break(Addr),                  // break <word>
    Run,                          // run
    Step,                         // step
    StoreMem(Addr, Byte),         // set mem <word> <byte>
    StoreReg(Register, Byte),     // set reg [AXY(PS)] <byte>
    StoreRegLong(Register, Addr), // set reg [(PC)(SP)] <word>
    LoadMem(Addr),                // get mem <word>
    LoadReg(Register),            // get reg [AXY(PS)(PC)(SP)]
    Nothing,
    ShowRegs,
}

impl TryFrom<&str> for Addr {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = if value.starts_with("0x") {
            u16::from_str_radix(value.strip_prefix("0x").unwrap(), 16)
        } else if value.starts_with("0b") {
            u16::from_str_radix(value.strip_prefix("0b").unwrap(), 2)
        } else {
            value.parse::<u16>()
        };

        Ok(Self(val.map_err(|e| e.to_string())?))
    }
}

impl TryFrom<&str> for Byte {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = if value.starts_with("0x") {
            u8::from_str_radix(value, 16)
        } else if value.starts_with("0b") {
            u8::from_str_radix(value, 2)
        } else {
            value.parse::<u8>()
        };

        Ok(Self(val.map_err(|e| e.to_string())?))
    }
}

impl Command {
    pub fn get_cmd(rl: &mut DefaultEditor) -> Result<Self, String> {
        let readline = rl.readline("cmd> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                if line.is_empty() {
                    Ok(Self::Nothing)
                } else {
                    Self::parse_line(&line)
                }
            }
            Err(ReadlineError::Eof) => {
                std::process::exit(0);
            }
            Err(ReadlineError::Interrupted) => Err(String::new()),
            Err(err) => {
                println!("Error: {:?}", err);
                std::process::exit(1);
            }
        }
    }

    pub fn parse_line(line: &str) -> Result<Self, String> {
        use Command::*;
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["help"] => Ok(Help),
            ["break", addr] | ["b", addr] => Ok(Break(Addr::try_from(*addr)?)),
            ["run"] | ["r"] => Ok(Run),
            ["step"] | ["s"] => Ok(Step),
            ["set", "mem", addr, val] => {
                Ok(StoreMem(Addr::try_from(*addr)?, Byte::try_from(*val)?))
            }
            ["set", "reg", reg, val] => {
                let reg = Register::try_from(*reg)?;
                if matches!(reg, Register::PC | Register::SP) {
                    Ok(StoreRegLong(reg, Addr::try_from(*val)?))
                } else {
                    Ok(StoreReg(reg, Byte::try_from(*val)?))
                }
            }
            ["get", "mem", addr] => Ok(LoadMem(Addr::try_from(*addr)?)),
            ["get", "reg", reg] => Ok(LoadReg(Register::try_from(*reg)?)),
            ["regs"] => Ok(ShowRegs),
            [] => Ok(Nothing),
            cmd => Err(format!("Invalid cmd: {cmd:?}")),
        }
    }
}

const USAGE: &str = r#"
    help                    Print this message
    break <addr>            Set a breakpoint at address <addr>
    run                     Resume execution
    step                    Step through one instruction   
    set mem <addr> <val>    Store <val> at address <addr>
    set reg <reg> <val>     Store <val> into register <reg> (A(u8), X(u8), Y(u8), PS(u8), PC(u16), SP(u8))
    get mem <addr>          Load <val> from address <addr>
    get reg <reg>           Load <val> from register <reg> (A, X, Y, PS, PC, SP)
"#;
