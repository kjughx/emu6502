use emu_6502::{hardware::bus, hardware::cpu, types::Addr};
use std::path::Path;

use clap::Parser;

mod visualize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, requires = "address")]
    load: Option<String>,

    #[arg(long)]
    address: Option<String>,
}

#[allow(arithmetic_overflow)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut bus = bus::Bus::new();
    bus.init(Addr::from(0xfffc), vec![0x00, 0x80]);

    if let Some(file) = args.load {
        let _addr: String = args.address.unwrap();
        let addr = if _addr.contains("0x") {
            u16::from_str_radix(_addr.strip_prefix("0x").unwrap(), 16)?
        } else {
            u16::from_str_radix(&_addr, 16)?
        };

        if addr < 255 || addr >= 0xff00 {
            return Err("Invalid start address: {addr}")?;
        }
        bus.load_file(Addr::from(addr), Path::new(&file))?;
    }

    let mut cpu = cpu::CPU::new(&mut bus);

    cpu.reset();

    visualize::run(&mut cpu)?;

    Ok(())
}
