use emu_6502::{hardware::bus, hardware::cpu, types::Addr};
use sdl2::{event::Event, keyboard::Keycode};

mod visualize;

#[allow(arithmetic_overflow)]
fn main() -> Result<(), String> {
    let mut bus = bus::Bus::new();
    bus.init(Addr::from(0xfffc), vec![0x00, 0x80]);
    bus.init(
        Addr::from(0x8000),
        vec![
            0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC, 0x00, 0x00, 0xA9,
            0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA,
        ],
    );

    let mut cpu = cpu::CPU::new(&mut bus);

    let (ctx, mut canvas, ttf) = visualize::new()?;
    let mut font = ttf.load_font("/usr/share/fonts/TTF/JetBrainsMono-Bold.ttf", 128)?;

    cpu.reset();

    // Execute one instruction at a time.
    let mut event_pump = ctx.event_pump()?;
    let mut update = true;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    cpu.exec();
                    update = true
                }
                Event::KeyDown {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    cpu.irq();
                    update = true
                }
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    cpu.nmi_irq();
                    update = true
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    cpu.reset();
                    update = true
                }
                _ => {}
            }
        }

        if update {
            visualize::update(&cpu, &mut canvas, &mut font)?;
            update = false;
        }
    }

    Ok(())
}
