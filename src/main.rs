extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

pub mod path_finder;

pub fn main() -> Result<(), String>
{
    // sdl setup
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let tile_size = 64;

    let screen_width = 13 * tile_size;
    let screen_height = 9 * tile_size;

    let window = video_subsystem
        .window("A* algoritm", screen_width, screen_height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    // calculate path first
    let tile_map = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    ];

    // rest of variables
    let tile_colors = HashMap::from([
        (0, Color::RGB(255, 255, 255)),
        (1, Color::RGB(45, 45, 45)),
    ]);

    let solid_tiles = [1];

    let start = Point::new(1, 1);
    let end = Point::new(5, 5);

    // start time
    let now = Instant::now();

    let path = path_finder::path_finder((start.x, start.y), (end.x, end.y), &tile_map, &solid_tiles).unwrap();

    // get and print elapsed time
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);


    let mut path_r = Rect::new(start.x * tile_size as i32, start.y * tile_size as i32, tile_size, tile_size);
    let mut index = 0;

    for v in &path
    {
        print!("{} ", v);
    }
    println!();

    let mut playing = -1;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    if playing == -1 {
                        playing = 1;
                        path_r = Rect::new(start.x * tile_size as i32, start.y * tile_size as i32, tile_size, tile_size);
                        index = 0;
                    }
                }
                _ => {}
            }
        }

        for x in 0..( screen_width / tile_size )
        {
            for y in 0..( screen_height / tile_size )
            {
                canvas.set_draw_color( tile_colors[ &tile_map[ y as usize ][ x as usize ]]);
                canvas.fill_rect( Rect::new((x * tile_size) as i32, (y * tile_size) as i32, tile_size, tile_size) )?;
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect( Rect::new(end.x * tile_size as i32, end.y * tile_size as i32, tile_size, tile_size) )?;

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect( Rect::new(start.x * tile_size as i32, start.y * tile_size as i32, tile_size, tile_size) )?;

        canvas.set_draw_color(Color::RGB(255, 0, 255));
        canvas.fill_rect(path_r)?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for x in 0..( screen_width / tile_size )
        {
            canvas.draw_line(Point::new((x * tile_size) as i32, 0), Point::new((x * tile_size) as i32, (screen_height) as i32))?;
        }
        for y in 0..( screen_height / tile_size )
        {
            canvas.draw_line(Point::new(0, (y * tile_size) as i32), Point::new((screen_width) as i32, (y * tile_size) as i32))?;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        let ts = tile_size as i32;
        let mut cp = Point::new(start.x * ts + ts/2, start.y * ts + ts/2);
        for i in &path
        {
            match i {
                0 => {
                    canvas.draw_line(cp, Point::new(cp.x - ts, cp.y - ts))?;
                    cp = Point::new(cp.x - ts, cp.y - ts);
                },
                1 => {
                    canvas.draw_line(cp, Point::new(cp.x, cp.y - ts))?;
                    cp = Point::new(cp.x, cp.y - ts);
                },
                2 => {
                    canvas.draw_line(cp, Point::new(cp.x + ts, cp.y - ts))?;
                    cp = Point::new(cp.x + ts, cp.y - ts);
                },
                3 => {
                    canvas.draw_line(cp, Point::new(cp.x + ts, cp.y))?;
                    cp = Point::new(cp.x + ts, cp.y);
                },
                4 => {
                    canvas.draw_line(cp, Point::new(cp.x + ts, cp.y + ts))?;
                    cp = Point::new(cp.x + ts, cp.y + ts);
                },
                5 => {
                    canvas.draw_line(cp, Point::new(cp.x, cp.y + ts))?;
                    cp = Point::new(cp.x, cp.y + ts);
                },
                6 => {
                    canvas.draw_line(cp, Point::new(cp.x - ts, cp.y + ts))?;
                    cp = Point::new(cp.x - ts, cp.y + ts);
                },
                7 => {
                    canvas.draw_line(cp, Point::new(cp.x - ts, cp.y))?;
                    cp = Point::new(cp.x - ts, cp.y);
                },
                _ => (),
            }
        }

        canvas.present();

        if index < path.len() && playing == 1
        {
            match path[index]
            {
                0 => {
                    path_r.x -= tile_size as i32;
                    path_r.y -= tile_size as i32;
                },
                1 => {
                    path_r.y -= tile_size as i32;
                },
                2 => {
                    path_r.x += tile_size as i32;
                    path_r.y -= tile_size as i32;
                },
                3 => {
                    path_r.x += tile_size as i32;
                },
                4 => {
                    path_r.x += tile_size as i32;
                    path_r.y += tile_size as i32;
                },
                5 => {
                    path_r.y += tile_size as i32;
                },
                6 => {
                    path_r.x -= tile_size as i32;
                    path_r.y += tile_size as i32;
                },
                7 => {
                    path_r.x -= tile_size as i32;
                },
                _ => (),
            }
            index += 1;
        } else {
            playing = -1
        }
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }

    Ok(())
}
