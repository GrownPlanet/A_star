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

    // the paths of neighbours of current
    let mut path_to_current_1: Vec<u32>;
    let mut path_to_current_2: Vec<u32>;
    let mut path_to_current_3: Vec<u32>;
    let mut path_to_current_4: Vec<u32>;

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
        let neighbours = [
            Node::calculate(&Point::new(current.location.x + 1, current.location.y), &start, &end, path_to_current_1),
            Node::calculate(&Point::new(current.location.x - 1, current.location.y), &start, &end, path_to_current_2),
            Node::calculate(&Point::new(current.location.x, current.location.y + 1), &start, &end, path_to_current_3),
            Node::calculate(&Point::new(current.location.x, current.location.y - 1), &start, &end, path_to_current_4),
        ];

        'f: for neighbour in neighbours {
            // checking if the neighbour is closed
            for node in &closed {
                if node.location == neighbour.location {
                    continue 'f;
                }
            }

            if neighbour.location.y < 0 || neighbour.location.x < 0 ||
                neighbour.location.y >= tile_map.len() as i32 || neighbour.location.x >= tile_map[0].len() as i32 {
                    continue 'f;
                }

            // check if tile is solid 
            for val in solid_tiles {
                if &tile_map[neighbour.location.y as usize][neighbour.location.x as usize] == val {
                    continue 'f;
                }
            }

            open.push(neighbour);
        }
    }
}
