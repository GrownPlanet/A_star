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
        .window("spaghetti code go brrrrr", 800, 600)
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
    let end = Point::new(1, 5);

    let mut event_pump = sdl_context.event_pump()?;

    let a = path_finder(start, end, &tile_map);
    for v in a.iter()
    {
        print!("{} ", v);
    }

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

fn square(a: i32) -> i32 { a * a }

struct Node
{
    pub location: Point,
    pub g_cost: u32, // how far away from start
    pub h_cost: u32, // how far away from end
    pub f_cost: u32, // g and h cost combined 
    pub path_to_parrent: Vec<u32>,
}

impl Node
{
    fn calculate(point: Point, start: Point, end: Point, path_to_parrent: Vec<u32>) -> Node
    {
        let g_cost = (square(point.x - start.x) as f32 + square(point.y - start.y) as f32).sqrt() as u32;
        let h_cost = (square(point.x - end.x) as f32 + square(point.y - end.y) as f32).sqrt() as u32;
        let f_cost = g_cost + h_cost;
        return Node
        {
            location: point,
            g_cost,
            h_cost,
            f_cost,
            path_to_parrent,
        };
    }
    fn compare(node1: &Node, node2: &Node) -> bool
    {
        node1.g_cost == node2.g_cost && node1.h_cost == node2.h_cost && node1.location == node2.location
    }
}

fn path_finder(start: Point, end: Point, _tile_map: &Vec<Vec<u32>>) -> Vec<u32>
{
    let mut open: Vec<Node> = vec![];
    let mut closed: Vec<Node> = vec![];

    open.push(Node::calculate(start, start, end, vec![]));

    let mut current;

    let mut vec_n_1: Vec<u32>;
    let mut vec_n_2: Vec<u32>;
    let mut vec_n_3: Vec<u32>;
    let mut vec_n_4: Vec<u32>;

    loop {
        current = Node::calculate(open[0].location, start, end, open[0].path_to_parrent.clone());
        // node with the lowest f cost
        for node in &open
        {
            println!("{}, {}", current.f_cost, node.f_cost);
            if node.f_cost < current.f_cost
            {
                current = Node::calculate(Point::new(node.location.x, node.location.y), start, end, node.path_to_parrent.clone());
            }
            else if node.h_cost < current.f_cost
            {
                current = Node::calculate(Point::new(node.location.x, node.location.y), start, end, node.path_to_parrent.clone());
            }
        }

        // remove from open list
        for index in 1..open.len()
        {
            if Node::compare(&current, &open[index - 1])
            {
                open.remove(index);
            }
        }

        // add to closed list
        closed.push(Node::calculate(Point::new(current.location.x, current.location.y), start, end, current.path_to_parrent.clone()));



        if current.location == end
        {
            let mut p = vec![];

            for v in 0..current.path_to_parrent.len()
            {
                p.push(current.path_to_parrent[v]);
            }

            return p;
        }

        // compare to neighbours
        vec_n_1 = current.path_to_parrent.clone();
        vec_n_1.push(2);

        vec_n_2 = current.path_to_parrent.clone();
        vec_n_2.push(4);


        vec_n_3 = current.path_to_parrent.clone();
        vec_n_3.push(3);

        vec_n_4 = current.path_to_parrent.clone();
        vec_n_4.push(1);
        
        let neightbours = [
            Node::calculate(Point::new(current.location.x + 1, current.location.y), start, end, vec_n_1),
            Node::calculate(Point::new(current.location.x - 1, current.location.y), start, end, vec_n_2),
            Node::calculate(Point::new(current.location.x, current.location.y + 1), start, end, vec_n_3),
            Node::calculate(Point::new(current.location.x, current.location.y - 1), start, end, vec_n_4),
        ];

        'l: for neighbour in neightbours
        {
            // check if neighbour is in closed
            for close in &closed
            {
                if Node::compare(&neighbour, close)
                {
                    break 'l;
                }
            }

            for node in &open
            {
                if Node::compare(&neighbour, node)
                {
                    if neighbour.f_cost > node.f_cost
                    {
                        break 'l;
                    }
                }
            }
            open.push(neighbour);
        }
    }
}
