extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use std::time::Duration;

pub fn main() -> Result<(), String> {
    // initialize
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("pong demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump()?;

    let rect = Rect::new(50, 100, 20, 150);

    'running: loop {
        // event
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // render
        // background
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        // rectangle
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(rect)?;
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
