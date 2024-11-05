use emu_6502::hardware::cpu::{
    instructions::{get_instruction, AddressingMode, Instruction},
    Flag, Register, CPU, STACK_START,
};
use emu_6502::types::Addr;
use sdl2::surface::Surface;
use sdl2::{event::Event, keyboard::Keycode};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf, video::Window};
use sdl2::{
    ttf::{Font, Sdl2TtfContext},
    Sdl,
};

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

macro_rules! surface(
    ($x:expr, $y:expr) => (
        sdl2::surface::Surface::new($x as u32, $y as u32, sdl2::pixels::PixelFormatEnum::RGBA32)
    )
);

pub const WIDTH: i32 = 1600;
pub const HEIGHT: i32 = 800;
pub const CW: i32 = 12;
pub const CH: i32 = 20;
pub const L: i32 = CH;
pub const REGISTER_RIGHT: i32 = WIDTH - 30 * CW;
pub const STACK_RIGHT: i32 = WIDTH - 60 * CW;
pub const MEMORY_LEFT: i32 = 5 * CW;
pub const PROMPT_X: i32 = 750;
pub const PROMPT_Y: i32 = 100;
pub const PROMPT_WIDTH: i32 = 500;
pub const PROMPT_HEIGHT: i32 = 6 * CH;
pub const PROMPT_MARGIN: i32 = 5;
pub const TEXT_BOX_X: i32 = PROMPT_WIDTH / 2 - 200;
pub const TEXT_BOX_Y: i32 = PROMPT_HEIGHT - CH * 4;
pub const TEXT_BOX_WIDTH: i32 = 200;
pub const TEXT_BOX_HEIGHT: i32 = CH * 3;

pub const BACKGROUND_COLOR: Color = Color::RGBA(0, 0, 0, 255); // BLACK
pub const TEXT_COLOR: Color = Color::RGBA(34, 180, 85, 255);

fn prompt(txt: &str, canvas: &mut Canvas<Window>, font: &mut Font) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    let mut surface = surface!(
        PROMPT_WIDTH + 2 * PROMPT_MARGIN,
        PROMPT_HEIGHT + 2 * PROMPT_MARGIN
    )?;
    surface.fill_rect(
        rect!(
            0,
            0,
            (PROMPT_WIDTH + PROMPT_MARGIN),
            (PROMPT_HEIGHT + PROMPT_MARGIN)
        ),
        BACKGROUND_COLOR,
    )?;
    surface.fill_rect(
        rect!(
            PROMPT_MARGIN,
            PROMPT_MARGIN,
            PROMPT_WIDTH - PROMPT_MARGIN,
            PROMPT_HEIGHT - PROMPT_MARGIN
        ),
        BACKGROUND_COLOR,
    )?;

    blit_text(txt, font, &mut surface, CW, CH)?;

    surface.fill_rect(
        rect!(TEXT_BOX_X, TEXT_BOX_Y, TEXT_BOX_WIDTH, TEXT_BOX_HEIGHT),
        BACKGROUND_COLOR,
    )?;

    let texture = texture_creator
        .create_texture_from_surface(surface)
        .map_err(|e| e.to_string())?;

    canvas.copy(
        &texture,
        None,
        rect!(
            PROMPT_X,
            PROMPT_Y,
            PROMPT_WIDTH + 2 * PROMPT_MARGIN,
            PROMPT_HEIGHT + 2 * PROMPT_MARGIN
        ),
    )?;

    Ok(())
}

pub fn run(mut cpu: CPU) -> Result<(), String> {
    let (ctx, mut canvas, ttf) = new()?;
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
    let mut font = ttf.load_font("/usr/share/fonts/TTF/JetBrainsMono-Bold.ttf", 128)?;
    let mut event_pump = ctx.event_pump()?;
    let mut refresh = true;
    let mut input = false;
    let mut memory_view_start = 0x7f00;
    let mut input_buffer = vec![];
    let mut prompt_text: &str = "";

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
                    refresh = true
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } if input => {
                    input = false;
                    let saddr = input_buffer.join("");
                    if saddr.starts_with("0X") {
                        if let Ok(_start) =
                            u16::from_str_radix(saddr.strip_prefix("0X").unwrap(), 16)
                        {
                            memory_view_start = _start;
                        }
                    } else if let Ok(_start) = u16::from_str_radix(&saddr, 16) {
                        memory_view_start = _start;
                    }
                    input_buffer = vec![];
                    refresh = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    input_buffer.pop();
                    refresh = true;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } if input => {
                    let key = keycode.to_string();
                    input_buffer.push(key);
                    refresh = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    cpu.reset();
                    refresh = true
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Slash),
                    ..
                } => {
                    input = true;
                    prompt_text = "Specify start address of memory view:";
                    refresh = true;
                }
                _ => {}
            }
        }

        if refresh {
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.clear();
            update(&cpu, &mut canvas, &mut font, memory_view_start)?;
            if input {
                prompt(prompt_text, &mut canvas, &mut font)?;
                if !input_buffer.is_empty() {
                    let mut surface = surface!(TEXT_BOX_WIDTH, TEXT_BOX_HEIGHT)?;
                    surface.fill_rect(None, BACKGROUND_COLOR)?;
                    blit_text(&input_buffer.join(""), &font, &mut surface, 0, CH)?;
                    let texture_creator = canvas.texture_creator();
                    let texture = texture_creator
                        .create_texture_from_surface(surface)
                        .map_err(|e| e.to_string())?;
                    canvas.copy(
                        &texture,
                        None,
                        rect!(
                            PROMPT_X + TEXT_BOX_X,
                            PROMPT_Y + TEXT_BOX_Y,
                            TEXT_BOX_WIDTH,
                            TEXT_BOX_HEIGHT
                        ),
                    )?;
                }
            }
            canvas.present();
            refresh = false;
        }
    }

    Ok(())
}

pub fn new() -> Result<(Sdl, Canvas<Window>, Sdl2TtfContext), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("emu6502", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;

    Ok((sdl_context, canvas, ttf_context))
}

pub fn update(
    cpu: &CPU,
    canvas: &mut Canvas<Window>,
    font: &mut Font,
    view_memory_start: u16,
) -> Result<(), String> {
    let mut current_line = 1;
    let mut surface = surface!(WIDTH, HEIGHT)?;

    // Title
    blit_text(
        "6502 CPU",
        font,
        &mut surface,
        WIDTH / 2 - 4,
        current_line * L,
    )?;

    current_line = 3;

    // Registers
    font.set_style(sdl2::ttf::FontStyle::UNDERLINE);
    blit_text(
        "REGISTERS",
        font,
        &mut surface,
        REGISTER_RIGHT + 5 * CW,
        current_line * L,
    )?;
    font.set_style(ttf::FontStyle::NORMAL);

    current_line += 2;

    let (instruction, addressing_mode): (Instruction, AddressingMode) =
        get_instruction(cpu.read(cpu.get_pc()));
    let arg = addressing_mode.get(cpu);
    let txt = format!("PC: {:#06X} -> {} {}", cpu.get_pc().0, instruction, arg);
    blit_text(&txt, font, &mut surface, REGISTER_RIGHT, current_line * L)?;
    current_line += 1;

    let txt = format!(
        "A: {:#04X},  X: {:#04X}",
        cpu.get_reg(Register::A).0,
        cpu.get_reg(Register::X).0
    );
    blit_text(&txt, font, &mut surface, REGISTER_RIGHT, current_line * L)?;
    current_line += 1;

    let txt = format!(
        "Y: {:#04X}, SP: {:#04X}",
        cpu.get_reg(Register::Y).0,
        cpu.get_sp().0
    );
    blit_text(&txt, font, &mut surface, REGISTER_RIGHT, current_line * L)?;

    current_line += 3;

    let txt = format!(
        "C: {}, Z: {}, I: {}, D: -",
        cpu.get_reg(Register::PS) & Flag::Carry,
        cpu.get_reg(Register::PS) & Flag::Zero,
        cpu.get_reg(Register::PS) & Flag::InterruptDisable,
    );
    blit_text(&txt, font, &mut surface, REGISTER_RIGHT, current_line * L)?;
    current_line += 1;

    let txt = format!(
        "B: {}, V: {}, N: {}",
        cpu.get_reg(Register::PS) & Flag::Break,
        cpu.get_reg(Register::PS) & Flag::Overflow,
        cpu.get_reg(Register::PS) & Flag::Negative
    );
    blit_text(&txt, font, &mut surface, REGISTER_RIGHT, current_line * L)?;

    // Stack
    current_line += 3;

    font.set_style(sdl2::ttf::FontStyle::UNDERLINE);
    blit_text(
        "STACK",
        font,
        &mut surface,
        STACK_RIGHT + 25 * CW,
        current_line * L,
    )?;
    font.set_style(ttf::FontStyle::NORMAL);
    current_line += 2;

    for byte in 0..16 {
        let data: Vec<String> = (0..16)
            .map(|b| format!("{:02x}", cpu.read(STACK_START + b + 16 * byte).0))
            .collect();
        blit_text(
            &format!(
                "${:04x}: {}",
                STACK_START.0 + (byte as u16) * 16,
                data.join(" ")
            ),
            font,
            &mut surface,
            STACK_RIGHT,
            L * (current_line + byte),
        )?;
    }

    current_line = 3;
    // // Remaining Memory
    font.set_style(sdl2::ttf::FontStyle::UNDERLINE);
    blit_text(
        "BUS",
        font,
        &mut surface,
        MEMORY_LEFT + 25 * CW,
        current_line * L,
    )?;
    font.set_style(ttf::FontStyle::NORMAL);
    current_line += 2;

    for byte in 0..16 {
        let data: Vec<String> = (0..16)
            .map(|b| format!("{:02x}", cpu.read(Addr::from(b + 16 * byte)).0))
            .collect();

        blit_text(
            &format!("${:04X}: {}", byte * 16, data.join(" ")),
            font,
            &mut surface,
            MEMORY_LEFT,
            L * (current_line + byte),
        )?;
    }
    current_line += 16;

    current_line += 1;
    for byte in 0..16 {
        let data: Vec<String> = (0..16)
            .map(|b| {
                format!(
                    "{:02x}",
                    cpu.read(Addr::from(view_memory_start + b + 16 * byte)).0
                )
            })
            .collect();
        blit_text(
            &format!("${:04X}: {}", view_memory_start + 16 * byte, data.join(" ")),
            font,
            &mut surface,
            MEMORY_LEFT,
            L * (current_line + byte as i32),
        )?;
    }

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .map_err(|e| e.to_string())?;
    canvas.copy(&texture, None, None)?;

    Ok(())
}

fn blit_text(
    txt: &str,
    font: &Font,
    dst: &mut Surface,
    x: i32,
    y: i32,
) -> Result<Option<Rect>, String> {
    let surface = font
        .render(txt)
        .blended(TEXT_COLOR)
        .map_err(|e| e.to_string())?;

    surface.blit_scaled(None, dst, rect!(x, y, (txt.len() as i32) * CW, CH))
}
