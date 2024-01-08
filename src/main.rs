use emu_6502::{bus, cpu};

fn main() {
    let mut bus = bus::Bus::new();
    let mut cpu = cpu::CPU::new(&mut bus);

    cpu.reset();

    // Execute one instruction at a time.
    loop {
        cpu.exec();
    }
}
