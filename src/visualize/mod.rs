use emu_6502::hardware::bus::STACK_START;
use emu_6502::hardware::cpu::{Flag, CPU};
use emu_6502::instruction::Instruction;
use emu_6502::types::Addr;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf, video::Window};
use sdl2::{
    ttf::{Font, Sdl2TtfContext},
    Sdl,
};

pub const WIDTH: u32 = 1600;
pub const HEIGHT: u32 = 800;
pub const CW: u32 = 12;
pub const CH: u32 = 20;
pub const L: i32 = CH as i32;
pub const REGISTER_RIGHT: i32 = WIDTH as i32 - 25 * CW as i32;
pub const STACK_RIGHT: i32 = WIDTH as i32 - 60 * CW as i32;
pub const MEMORY_LEFT: i32 = 5 * CW as i32;

pub fn new() -> Result<(Sdl, Canvas<Window>, Sdl2TtfContext), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("emu6502", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGBA(0, 0, 255, 255)); // BLUE
    canvas.clear();
    canvas.present();

    let ttf_context = ttf::init().map_err(|e| e.to_string())?;

    Ok((sdl_context, canvas, ttf_context))
}

pub fn update(cpu: &CPU, canvas: &mut Canvas<Window>, font: &mut Font) -> Result<bool, String> {
    canvas.set_draw_color(Color::RGBA(0, 0, 255, 255)); // BLUE
    canvas.clear();

    let texture_creator = canvas.texture_creator();
    let mut current_line;

    // Now begins the heavy lifting...
    // Title
    let (texture, width) = render_text("6502 CPU", &texture_creator, &font)?;
    canvas.copy(
        &texture,
        None,
        Some(Rect::new((WIDTH / 2 - width / 2) as i32, 0, width, CH)),
    )?;
    current_line = 3;

    // Registers
    font.set_style(sdl2::ttf::FontStyle::UNDERLINE);
    let (texture, width) = render_text("REGISTERS", &texture_creator, &font)?;
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(
            REGISTER_RIGHT + 5 * CW as i32,
            current_line * L,
            width,
            CH,
        )),
    )?;
    font.set_style(ttf::FontStyle::NORMAL);

    current_line += 2;

    let txt = format!(
        "PC: {:#06X} -> {}",
        cpu.pc.0,
        Instruction::from(cpu.read_memory(cpu.pc))
    );
    let (texture, width) = render_text(&txt, &texture_creator, &font)?;
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(REGISTER_RIGHT, current_line * L, width, CH)),
    )?;
    current_line += 1;

    let txt = format!("A: {:#04X}, X: {:#04X}", cpu.a.0, cpu.x.0);
    let (texture, width) = render_text(&txt, &texture_creator, &font)?;
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(REGISTER_RIGHT, current_line * L, width, CH)),
    )?;
    current_line += 1;

    let txt = format!("Y: {:#04X}, SP: {:#04X}", cpu.y.0, cpu.sp.0);
    let (texture, width) = render_text(&txt, &texture_creator, &font)?;
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(REGISTER_RIGHT, current_line * L, width, CH)),
    )?;
    current_line += 2;

    let txt = format!(
        "C: {}, Z: {}, I: {}, D: -",
        cpu.ps & Flag::Carry,
        cpu.ps & Flag::Zero,
        cpu.ps & Flag::InterruptDisable,
    );
    let (texture, width) = render_text(&txt, &texture_creator, &font)?;
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(REGISTER_RIGHT, current_line * L, width, CH)),
    )?;
    current_line += 1;

    let txt = format!(
        "B: {}, V: {}, N: {}",
        cpu.ps & Flag::BreakCmd,
        cpu.ps & Flag::Overflow,
        cpu.ps & Flag::Negative
    );
    let (texture, width) = render_text(&txt, &texture_creator, &font)?;
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(REGISTER_RIGHT, current_line * L, width, CH)),
    )?;
    current_line += 1;

    // Stack
    current_line += 1;

    font.set_style(sdl2::ttf::FontStyle::UNDERLINE);
    let (texture, width) = render_text("STACK", &texture_creator, &font)?;
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(
            STACK_RIGHT + 25 * CW as i32,
            current_line * L,
            width,
            CH,
        )),
    )?;
    font.set_style(ttf::FontStyle::NORMAL);
    current_line += 2;

    for byte in 0..16 {
        let data: Vec<String> = (0..16)
            .into_iter()
            .map(|b| {
                format!(
                    "{:02X}",
                    cpu.read_memory(Addr::from(STACK_START + 8 * byte + b)).0
                )
            })
            .collect();
        let (texture, width) = render_text(
            &format!(
                "${:04X}: {}",
                STACK_START.0 + (byte as u16) * 16,
                data.join(" ")
            ),
            &texture_creator,
            &font,
        )?;
        canvas.copy(
            &texture,
            None,
            Some(Rect::new(
                STACK_RIGHT,
                L * (current_line + byte as i32),
                width,
                CH,
            )),
        )?;
    }

    current_line = 3;
    // Remaining Memory
    font.set_style(sdl2::ttf::FontStyle::UNDERLINE);
    let (texture, width) = render_text("MEMORY", &texture_creator, &font)?;
    canvas.copy(
        &texture,
        None,
        Some(Rect::new(
            MEMORY_LEFT + 25 * CW as i32,
            current_line * L,
            width,
            CH,
        )),
    )?;
    font.set_style(ttf::FontStyle::NORMAL);
    current_line += 2;

    for byte in 0..16 {
        let data: Vec<String> = (0..16)
            .into_iter()
            .map(|b| format!("{:02X}", cpu.read_memory(Addr::from(8 * byte + b)).0))
            .collect();
        let (texture, width) = render_text(
            &format!("${:04X}: {}", byte * 16, data.join(" ")),
            &texture_creator,
            &font,
        )?;
        canvas.copy(
            &texture,
            None,
            Some(Rect::new(
                MEMORY_LEFT,
                L * (current_line + byte as i32),
                width,
                CH,
            )),
        )?;
    }
    current_line += 16;

    current_line += 1;
    for byte in 0..16 {
        let data: Vec<String> = (0..16)
            .into_iter()
            .map(|b| {
                format!(
                    "{:02x}",
                    cpu.read_memory(Addr::from(u16::MAX - 255 + 8 * byte + b)).0
                )
            })
            .collect();
        let (texture, width) = render_text(
            &format!("${:04x}: {}", u16::MAX - 255 + byte * 16, data.join(" ")),
            &texture_creator,
            &font,
        )?;
        canvas.copy(
            &texture,
            None,
            Some(Rect::new(
                MEMORY_LEFT,
                L * (current_line + byte as i32),
                width,
                CH,
            )),
        )?;
    }

    canvas.present();

    Ok(true)
}

fn render_text<'a>(
    txt: &str,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &Font,
) -> Result<(Texture<'a>, u32), String> {
    let surface = font
        .render(txt)
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    Ok((texture, txt.len() as u32 * CW))
}
