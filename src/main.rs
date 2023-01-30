use std::fs::File;
use std::io::Read;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use std::time::Duration;
use sdl2::rect::Rect;

mod cpu;
mod memory;

const WIDTH : usize = 64;
const HEIGHT : usize = 32;

fn load_program(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    return buffer;
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 64*10*2, 32*10*2)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut timer = sdl_context.timer().expect("sdl context timer failed");
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    let mut cpu = cpu::CPU::new();
    // Load program into buffer
    let buffer = load_program(&String::from("src\\ibm_logo.ch8"));
    // Load buffer to memory
    cpu.load_buffer_to_memory(buffer);

    'running: loop {
        let mut event_pump = sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let key_state = KeyboardState::new(&mut event_pump);
        cpu.keypad[0x0] = key_state.is_scancode_pressed(Scancode::T) as u8;
        cpu.keypad[0x1] = key_state.is_scancode_pressed(Scancode::Y) as u8;
        cpu.keypad[0x2] = key_state.is_scancode_pressed(Scancode::U) as u8;
        cpu.keypad[0x3] = key_state.is_scancode_pressed(Scancode::G) as u8;
        cpu.keypad[0x4] = key_state.is_scancode_pressed(Scancode::H) as u8;
        cpu.keypad[0x5] = key_state.is_scancode_pressed(Scancode::J) as u8;
        cpu.keypad[0x6] = key_state.is_scancode_pressed(Scancode::B) as u8;
        cpu.keypad[0x7] = key_state.is_scancode_pressed(Scancode::N) as u8;
        cpu.keypad[0x8] = key_state.is_scancode_pressed(Scancode::M) as u8;
        cpu.keypad[0x9] = key_state.is_scancode_pressed(Scancode::Q) as u8;
        cpu.keypad[0xA] = key_state.is_scancode_pressed(Scancode::W) as u8;
        cpu.keypad[0xB] = key_state.is_scancode_pressed(Scancode::E) as u8;
        cpu.keypad[0xC] = key_state.is_scancode_pressed(Scancode::A) as u8;
        cpu.keypad[0xD] = key_state.is_scancode_pressed(Scancode::S) as u8;
        cpu.keypad[0xE] = key_state.is_scancode_pressed(Scancode::D) as u8;
        cpu.keypad[0xF] = key_state.is_scancode_pressed(Scancode::Z) as u8;

        cpu.tick();

        let clamp_pos = 20;
        let clamp_size = 20;

        if cpu.should_draw() {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if cpu.screen[y][x] {
                        // Foreground
                        canvas.set_draw_color(Color::RGB(251, 241, 199));
                    } else {
                        // Background
                        canvas.set_draw_color(Color::RGB(69, 133, 149));
                    }
                    // x, y, w, h
                    canvas
                        .fill_rect(Rect::new(
                            x as i32 * clamp_pos,
                            y as i32 * clamp_pos,
                            clamp_size as u32,
                            clamp_size as u32,
                        ))
                        .unwrap();
                }
            }
            canvas.present();
            cpu.set_draw(true);
        }
        
        canvas.present();

        let now = timer.ticks();
        let dt = now - before;

        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }
        before = now;
        fps += 1;

        if now - last_second > 1000 {
            last_second = now;
            fps = 0;
        }

        cpu.tick_timer();
    }   
}