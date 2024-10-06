use clap::Parser;
use emu_6502::hardware::display::Display;
use emu_6502::hardware::keyboard::Keyboard;
use emu_6502::hardware::rom::Rom;
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

    let mut bus = Bus::new(); // Everyone talks over this

    let memory = Mutex!(Memory::default());
    let rom = Mutex!(Rom::new(args.load));

    let keyboard = Mutex!(Keyboard::new());
    {
        let _keyboard = keyboard.clone();
        std::thread::spawn(|| {
            Keyboard::poll(_keyboard);
        });
    }

    let display = Mutex!(Display::new());

    bus.register(memory)?;
    bus.register(keyboard)?;
    bus.register(display)?;
    bus.register(rom)?;

    let cpu = Mutex!(CPU::new(Mutex!(bus)));
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
