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
    // let args = Args::parse();
    //
    // let mut bus = Bus::new(); // Everyone talks over this
    //
    // let memory = Mutex!(Memory::new(MEMORY_SIZE));
    // let rom = Mutex!(Rom::new(args.load));
    //
    // let keyboard = Mutex!(Keyboard::new());
    // {
    //     let _keyboard = keyboard.clone();
    //     std::thread::spawn(|| {
    //         poll(_keyboard);
    //     });
    // }
    //
    // let display = Mutex!(Display::new());
    //
    // bus.register(memory, MEMORY_START, MEMORY_SIZE)?;
    // bus.register(keyboard, Addr(0x5000), Addr(0x5001))?;
    // bus.register(display, Addr(0x5002), Addr(0x5003))?;
    // bus.register(rom, ROM_START, Addr(ROM_START.0 + ROM_SIZE.0))?;
    //
    // let cpu = Mutex!(CPU::new(Mutex!(bus)));
    // cpu.lock().unwrap().reset();
    //
    // if args.visualize {
    //     visualize::run(cpu)?;
    // } else {
    //     loop {
    //         if !cpu.lock().unwrap().exec() {
    //             break;
    //         }
    //     }
    //     visualize::run(cpu)?;
    // }

    let bus = Mutex!(Bus::new());
    let memory = Mutex!(Memory::new(Addr(0xffff)));
    bus.lock()
        .unwrap()
        .register(memory, Addr(0x0000), Addr(0xffff))
        .unwrap();

    for (i, byte) in include_bytes!("hardware/cpu/instructions/arithmetic.bin").iter().enumerate() {
        bus.lock().unwrap().write(Addr(i as u16), Byte(*byte));
    }

    let cpu = Mutex!(CPU::new(bus));
    cpu.lock().unwrap().set_pc(Addr(0x0400));

    let mut instructions = 0;

    loop {
        if !cpu.lock().unwrap().exec() {
            break;
        }
        instructions += 1;
        // assert!(instructions <= 5475855, "Too many instructions!");
    }
    dbg!(instructions);

    assert_eq!(
        cpu.lock().unwrap().get_pc(),
        Addr(0x0459),
        "Failure: {:#06X}",
        cpu.lock().unwrap().get_pc().0
    );

    Ok(())
}
