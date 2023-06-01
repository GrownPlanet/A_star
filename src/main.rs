extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use std::collections::HashMap;
use std::time::Duration;

pub fn main() -> Result<(), String> {
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

    let tile_map = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let tile_colors = HashMap::from([
        (0, Color::RGB(255, 255, 255)),
        (1, Color::RGB(45, 45, 45)),
    ]);

    let solid_tiles = [1];

    let start = Point::new(1, 1);
    let end = Point::new(5, 5);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for x in 0..( screen_width / tile_size )
        {
            canvas.draw_line(Point::new((x * tile_size) as i32, 0), Point::new((x * tile_size) as i32, (screen_height) as i32))?;
        }
        for y in 0..( screen_height / tile_size )
        {
            canvas.draw_line(Point::new(0, (y * tile_size) as i32), Point::new((screen_width) as i32, (y * tile_size) as i32))?;
        }

        canvas.present();
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

pub mod path_finder
{
    extern crate sdl2;

    use sdl2::rect::Point;

    fn square_i32( num: i32 ) -> i32 { num * num }

    struct Node
    {
        location: Point,
        h_cost: u32, // distance to the end
        f_cost: u32, // g (distance to the start) and h cost combined 
        path_to_parrent: Vec<u32>,
    }
    
    impl Node
    {
        fn calculate( location: Point, start: Point, end: Point, path_to_parrent: Vec<u32> ) -> Node
        {
            let g_cost =  ( ((square_i32(location.x - start.x) + square_i32(location.y - start.y)) as f32).sqrt() ) as u32;
            let h_cost =  ( ((square_i32(location.x - end.x) + square_i32(location.y - end.y)) as f32).sqrt() ) as u32;
            let f_cost = g_cost + h_cost;

            return Node
            {
                location,
                h_cost,
                f_cost,
                path_to_parrent,
            };
        }

        fn compare(node1: &Node, node2: &Node) -> bool
        {
            node1.location == node2.location && node1.path_to_parrent == node2.path_to_parrent
        }
    }

    pub fn path_finder (start: Point, end: Point, tile_map: &Vec<Vec<u32>>, solid_tiles: &[u32]) -> Vec<u32>
    {
        // list with values that can be used
        let mut open: Vec<Node> = Vec::new();
        // list with used values
        let mut closed: Vec<Node> = Vec::new();

        // putting the start node in open
        open.push(Node::calculate(start, start, end, Vec::new()));

        // the current value
        let mut current: Node;
        
        // the paths of neighbours of current
        let mut path_to_current_1: Vec<u32>;
        let mut path_to_current_2: Vec<u32>;
        let mut path_to_current_3: Vec<u32>;
        let mut path_to_current_4: Vec<u32>;

        let mut neighbours = Vec::new();

        // the main loop
        loop
        {
            // setting the current value to the lowest value in open
            current = Node::calculate(open[0].location, start, end, open[0].path_to_parrent.clone());

            for node in &open
            {
                if node.f_cost < current.f_cost
                {
                    current = Node::calculate(node.location, start, end, node.path_to_parrent.clone());
                }
            }

            // remove the current node from open
            let node_index = open.iter().position(|n| Node::compare(n, &current)).unwrap();
            open.remove(node_index);

            // add open to the closed list
            closed.push(Node::calculate(current.location, start, end, current.path_to_parrent.clone()));

            if current.location == end
            {
                return current.path_to_parrent;
            }

            // creating the neighbours of current paths
            path_to_current_1 = current.path_to_parrent.clone();
            path_to_current_1.push(2);

            path_to_current_2 = current.path_to_parrent.clone();
            path_to_current_2.push(4);

            path_to_current_3 = current.path_to_parrent.clone();
            path_to_current_3.push(3);

            path_to_current_4 = current.path_to_parrent.clone();
            path_to_current_4.push(1);

            // creating the neighbours 
            neighbours.clear();

            neighbours.push(Node::calculate(Point::new(current.location.x + 1, current.location.y), start, end, path_to_current_1));
            neighbours.push(Node::calculate(Point::new(current.location.x - 1, current.location.y), start, end, path_to_current_2));
            neighbours.push(Node::calculate(Point::new(current.location.x, current.location.y + 1), start, end, path_to_current_3));
            neighbours.push(Node::calculate(Point::new(current.location.x, current.location.y - 1), start, end, path_to_current_4));

            //
            'f: for neighbour in &neighbours
            {
                // checking if the neighbour is closed
                for node in &closed
                {
                    if node.location == neighbour.location
                    {
                        continue 'f;
                    }
                }

                if neighbour.location.y < 0 || neighbour.location.x < 0 ||
                    neighbour.location.y >= tile_map.len() as i32 || neighbour.location.x >= tile_map[0].len() as i32
                {
                    continue 'f;
                }

                // check if tile is solid 
                for val in solid_tiles
                {
                    if &tile_map[neighbour.location.y as usize][neighbour.location.x as usize] == val
                    {
                        continue 'f;
                    }
                }

                open.push(neighbour);
            }
        }
    }
}
