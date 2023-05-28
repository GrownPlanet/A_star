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
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let start = Point::new(1, 1);
    let end = Point::new(5, 5);

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

        // drawing the tilemap
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

        canvas.set_draw_color(Color::RGB(255, 255, 0));
        canvas.fill_rect(Rect::new(start.x * 100, start.y * 100, 100, 100))?;

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.fill_rect(Rect::new(end.x * 100, end.y * 100, 100, 100))?;

        // drawing a grid
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

fn square(a: u32) -> u32 { a * a }

struct Node
{
    pub g_cost: u32, // how far away from start
    pub h_cost: u32, // how far away from end
    pub f_cost: u32, // g and h cost combined 
}

fn calculate_node(point: Point, start: Point, end: Point) -> Node
{
    let g_cost = (square((point.x - start.x) as u32) as f32 + square((point.y - start.y) as u32) as f32).sqrt() as u32;
    let h_cost = (square((point.x - end.x) as u32) as f32 + square((point.y - end.y) as u32) as f32).sqrt() as u32;
    let f_cost = g_cost + h_cost;
    return Node
    {
        g_cost,
        h_cost,
        f_cost
    };
}

fn path_finder(start: Point, end: Point)
{
    let mut open: Vec<Node> = vec![];
    let mut closed: Vec<Node> = vec![];

    open.push(calculate_node(start, start, end));

    loop {
        //let current_node = sqrtf32(x)
    }

}
