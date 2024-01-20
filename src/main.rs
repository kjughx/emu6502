use clap::Parser;
use emu_6502::hardware::display::Display;
use emu_6502::hardware::keyboard::{poll, Keyboard};
use emu_6502::hardware::memory::{MEMORY_SIZE, MEMORY_START};
use emu_6502::hardware::rom::{Rom, ROM_SIZE, ROM_START};
use emu_6502::types::*;
use emu_6502::Mutex;
use emu_6502::{hardware::bus::Bus, hardware::cpu::CPU, hardware::memory::Memory};

use std::sync::{Arc, Mutex};

mod visualize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    load: Option<String>,

    #[arg(long)]
    visualize: bool,
}

#[allow(arithmetic_overflow)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let bus = Mutex!(Bus::new()); // Everyone talks over this

    let memory = Mutex!(Memory::new());

    let rom = Mutex!(Rom::new(args.load));

    let keyboard = Mutex!(Keyboard::new(bus.clone()));
    {
        let _keyboard = keyboard.clone();
        std::thread::spawn(|| {
            poll(_keyboard);
        });
    }
    let display = Mutex!(Display::new());

    let cpu = Mutex!(CPU::new(bus.clone()));

    bus.lock()
        .unwrap()
        .register(memory, MEMORY_START, MEMORY_SIZE)?;

    bus.lock()
        .unwrap()
        .register(keyboard, Addr(0x5000), Addr(0x5001))?;
    bus.lock()
        .unwrap()
        .register(display, Addr(0x5002), Addr(0x5003))?;
    bus.lock()
        .unwrap()
        .register(rom, ROM_START, Addr(ROM_START.0 + ROM_SIZE.0))?;

    cpu.lock().unwrap().reset();

    if args.visualize {
        visualize::run(cpu)?;
    } else {
        loop {
            if !cpu.lock().unwrap().exec() {
                break;
            }
        }
        visualize::run(cpu)?;
    }

    Ok(())
}
