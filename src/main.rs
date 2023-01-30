use chip8_emulator::chip8;
use chip8_emulator::error::Error;
use chip8_emulator::Result;
use clap::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::sys::exit;
use sdl2::video::Window;
use sdl2::EventPump;
fn main() -> Result<()> {
    let app = Command::new("My app")
        .arg(arg!([file] "Path of your rom"))
        .get_matches();
    let path: String = app.get_one::<String>("file").expect("required").to_string();

    run(path)?;
    Ok(())
}

fn run(path: String) -> Result<()> {
    let sdl2_context = sdl2::init().map_err(|s| Error::SdlError(s))?;
    let video_subsystem = sdl2_context.video().map_err(|s| Error::SdlError(s))?;
    let window = video_subsystem
        .window("chip8_emulator", 640, 320)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl2_context.event_pump().map_err(|s| Error::SdlError(s))?;

    chip8::cpu_reset(path)?;

    let dura = std::time::Duration::from_millis(1);

    loop {
        canvas_draw(&mut canvas)?;
        unsafe {
            chip8::execute()?;
        }
        key_event(&mut event_pump);

        std::thread::sleep(dura);
    }

    Ok(())
}

fn canvas_draw(canvas: &mut Canvas<Window>) -> Result<()> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for i in 0..64 {
        for j in 0..32 {
            let pixel = chip8::view(i, j);
            if pixel == 1 {
                canvas.set_draw_color(Color::WHITE);
                canvas
                    .fill_rect(Rect::new(i as i32 * 10, j as i32 * 10, 10, 10))
                    .map_err(|s| Error::SdlError(s))?;
            }
        }
    }

    canvas.present();
    Ok(())
}

fn key_event(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::X => chip8::key_pressed(0),
                Keycode::Num1 => chip8::key_pressed(1),
                Keycode::Num2 => chip8::key_pressed(2),
                Keycode::Num3 => chip8::key_pressed(3),
                Keycode::Q => chip8::key_pressed(4),
                Keycode::W => chip8::key_pressed(5),
                Keycode::E => chip8::key_pressed(6),
                Keycode::A => chip8::key_pressed(7),
                Keycode::S => chip8::key_pressed(8),
                Keycode::D => chip8::key_pressed(9),
                Keycode::Z => chip8::key_pressed(10),
                Keycode::C => chip8::key_pressed(11),
                Keycode::Num4 => chip8::key_pressed(12),
                Keycode::R => chip8::key_pressed(13),
                Keycode::F => chip8::key_pressed(14),
                Keycode::V => chip8::key_pressed(15),
                Keycode::Escape => unsafe { exit(0) },
                _ => (),
            },
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::X => chip8::key_released(0),
                Keycode::Num1 => chip8::key_released(1),
                Keycode::Num2 => chip8::key_released(2),
                Keycode::Num3 => chip8::key_released(3),
                Keycode::Q => chip8::key_released(4),
                Keycode::W => chip8::key_released(5),
                Keycode::E => chip8::key_released(6),
                Keycode::A => chip8::key_released(7),
                Keycode::S => chip8::key_released(8),
                Keycode::D => chip8::key_released(9),
                Keycode::Z => chip8::key_released(10),
                Keycode::C => chip8::key_released(11),
                Keycode::Num4 => chip8::key_released(12),
                Keycode::R => chip8::key_released(13),
                Keycode::F => chip8::key_released(14),
                Keycode::V => chip8::key_released(15),
                _ => (),
            },
            _ => (),
        }
    }
}
