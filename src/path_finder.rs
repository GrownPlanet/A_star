fn square_i32( num: i32 ) -> i32 {
    num * num
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    fn clone(&self) -> Point {
        Point {
            x: self.x, 
            y: self.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Node {
    location: Point,
    f_cost: u32, // g (distance to the start) and h (distance to the end) cost combined 
    path_to_parrent: Vec<u32>,
}

impl Node {
    fn calculate( location: &Point, start: &Point, end: &Point, path_to_parrent: Vec<u32> ) -> Node {
        let g_cost =  ( ((square_i32(location.x - start.x) + square_i32(location.y - start.y)) as f32).sqrt() ) as u32;
        let h_cost =  ( ((square_i32(location.x - end.x) + square_i32(location.y - end.y)) as f32).sqrt() ) as u32;
        let f_cost = g_cost + h_cost;

        return Node {
            location: location.clone(),
            f_cost,
            path_to_parrent,
        };
    }

    fn compare(node1: &Node, node2: &Node) -> bool {
        node1.location == node2.location && node1.path_to_parrent == node2.path_to_parrent
    }

    fn clone(&self) -> Node {
        Node {
            location: self.location.clone(), 
            f_cost: self.f_cost,
            path_to_parrent: self.path_to_parrent.clone(),
        }
    }
}

pub fn path_finder (start: (i32, i32), end: (i32, i32), tile_map: &Vec<Vec<u32>>, solid_tiles: &[u32]) -> Result<Vec<u32>, String> {
    // create start and end points 
    let start = Point::new(start.0, start.1);
    let end = Point::new(end.0, end.1);

    // list with values that can be used
    let mut open: Vec<Node> = Vec::new();
    // list with used values
    let mut closed: Vec<Node> = Vec::new();

    // putting the start node in open
    open.push(Node::calculate(&start, &start, &end, Vec::new()));

    // the current value
    let mut current: Node;

    let mut path_to_parrent: Vec<u32>;

    let mut location: Point;
    let mut neighbour: Node;

    let directions = [
                (-1, -1),
                ( 0, -1),
                ( 1, -1),
                ( 1,  0),
                ( 1,  1),
                ( 0,  1),
                (-1,  1),
                (-1,  0)
    ];


    // the main loop
    loop {
        // check if there is no path 
        if open.len() == 0 {
            return Err(String::from("impossible path"));
        }
        // setting the current value to the lowest value in open
        current = open
            .iter()
            .min_by_key(|node| node.f_cost)
            .unwrap()
            .clone();

        // remove the current node from open
        open.retain(|node| !Node::compare(&current, node));

        // add open to the closed list
        closed.push(Node::calculate(&current.location, &start, &end, current.path_to_parrent.clone()));

        if current.location == end {
            return Ok(current.path_to_parrent);
        }

        // loop over all the neighbours 
        for (x_dir, y_dir) in directions.iter() {
            // set the location
            location = Point::new(current.location.x + x_dir, current.location.y + y_dir);

            // check if location is out of bounds
            if  location.y < 0 
                || location.x < 0 
                || location.y >= tile_map.len() as i32
                || location.x >= tile_map[0].len() as i32
            {
                    continue;
            }

            // check if location is on a solid tile
            if solid_tiles.contains(&tile_map[location.y as usize][location.x as usize]) {
                continue;
            }
            
            // clone the path to parrent vector
            path_to_parrent = current.path_to_parrent.clone();


            // add the right number
            /*
             * 0 1 2
             * 7   3
             * 6 5 4
            */
            match (x_dir, y_dir) {
                (-1, -1) => path_to_parrent.push(0),
                ( 0, -1) => path_to_parrent.push(1),
                ( 1, -1) => path_to_parrent.push(2),
                ( 1,  0) => path_to_parrent.push(3),
                ( 1,  1) => path_to_parrent.push(4),
                ( 0,  1) => path_to_parrent.push(5),
                (-1,  1) => path_to_parrent.push(6),
                (-1,  0) => path_to_parrent.push(7),
                _ => return Err(String::from("Can't find direction")),
            }

            // create the neighbour
            neighbour = Node::calculate(&location.clone(), &start, &end, path_to_parrent);

            // check if the neighbour is in closed
            if closed.iter().any(|node| neighbour.location == node.location) {
                continue;
            }

            // add the neighbour to open
            open.push(neighbour);
        }
    }
}
