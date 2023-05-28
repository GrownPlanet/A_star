extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let tile_map = vec![
        vec![1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
    ];

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for y in 0..6
        {
            for x in 0..8
            {
                match tile_map[y][x] {
                   0 => canvas.set_draw_color(Color::RGB(255, 255, 255)),
                   1 => canvas.set_draw_color(Color::RGB(0, 255, 0)),
                   _ => (),
                }
                canvas.fill_rect(Rect::new(x as i32 * 100, y as i32 * 100, 100, 100))?;
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for x in 0..8
        {
            canvas.draw_line(Point::new(x * 100, 0), Point::new(x*100, 600))?;
        }
        for y in 0..8
        {
            canvas.draw_line(Point::new(0, y*100), Point::new(800, y*100))?;
        }
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
