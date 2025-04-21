#![allow(arithmetic_overflow)]
use clap::Parser;
use e6502::debugger::Debugger;
use e6502::hardware::clock::Clock;
use e6502::hardware::display::Display;
use e6502::hardware::keyboard::Keyboard;
use e6502::hardware::rom::Rom;
use e6502::{hardware::bus::Bus, hardware::cpu::CPU, hardware::memory::Memory};
use std::sync::Arc;

// mod visualize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    load: Option<String>,

    #[arg(long)]
    visualize: bool,

    #[arg(long)]
    debug: bool,
}

#[allow(arithmetic_overflow)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut bus = Bus::new(); // Everyone talks over this

    let memory = Memory::default();
    let rom = Rom::new(args.load);

    let keyboard = Keyboard::new();
    let display = Display::new();
    let clk = Arc::new(Clock::new());

    bus.register(memory)?;
    bus.register(keyboard)?;
    bus.register(display)?;
    bus.register(rom)?;

    let mut cpu = CPU::new(bus, clk.clone());
    if args.debug {
        Debugger::new(cpu, true).start();
    }

    std::thread::spawn(move || {
        cpu.reset();
        println!("Ready, set, go!");
        loop {
            cpu.exec();
        }
    });

    loop {
        clk.tick();
        std::thread::sleep(std::time::Duration::from_secs(1));
        clk.wait_tock();
    }
}
