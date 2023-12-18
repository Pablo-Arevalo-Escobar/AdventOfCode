use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug,Hash,Clone,Eq,PartialEq)]
struct NodeRecord 
{
    pos : (i32,i32),
    direction : (i32, i32),
    block_length : u32,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Path {
    node: (i32, i32),
    cost: u32,
    block_length: u32, // Number of consecutive blocks in the same directio
    block_direction: (i32, i32),
    record : Vec<NodeRecord>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)            
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in contents.lines() {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(row);
    }

    for row in grid.iter() {
        println!("{:?}", row);
    }

    // need to do a queue to traverse the graph
    let start_node = (0, 0);
    let end_node = (grid.len() as i32 - 1, grid[0].len() as i32 - 1);
    let init_path = Path {
        node: start_node,
        cost: 0,
        block_length: 1,
        block_direction: (0, 0),
        record: Vec::new(),
    };

    // priority queue start
    let mut pq: BinaryHeap<Path> = BinaryHeap::new();
    let mut visited_map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut record : Vec<NodeRecord> = Vec::new();
    pq.push(init_path);

    while !pq.is_empty() {
        let curr_path = pq.pop().unwrap();
        //println!("cost : {}", curr_path.cost);

        //println!("{:?}", curr_path);
        println!("COST {}", curr_path.cost);
        print_record(&curr_path.record);
        if curr_path.node == end_node {
            println!("Total cost : {}", curr_path.cost);
            record = curr_path.record.clone();
            break;
        }

        // Add top, bot, left and right to the priority queue
        add_node(&grid, &mut pq, &mut visited_map, &curr_path, (0, 1));
        add_node(&grid, &mut pq, &mut visited_map, &curr_path, (0, -1));
        add_node(&grid, &mut pq, &mut visited_map, &curr_path, (1, 0));
        add_node(&grid, &mut pq, &mut visited_map, &curr_path, (-1, 0));
    }

    let mut cost_index : usize = 0;
    let mut debug_grid : Vec<Vec<char>> = Vec::new();
    for i in 0..=grid.len()-1 {
        let mut row : Vec<char> = Vec::new();
        for j in 0..=grid[0].len()-1 {
            row.push('.');
        }
        debug_grid.push(row);
    }

    for rec in record {
        let (i,j) = rec.pos;
        match rec.direction {
            (0,1) => {
                debug_grid[i as usize][j as usize] = '>';
            },
            (1,0) => {
                debug_grid[i as usize][j as usize] = 'v';
            },
            (-1,0) => {
                debug_grid[i as usize][j as usize] = '^';
            },
            (0,-1) => {
                debug_grid[i as usize][j as usize] = '<';
            },
            _ =>{} ,
        }
    }

    for row in debug_grid 
    {
        println!("{:?}", row);
    }
}

fn add_node(
    grid: &Vec<Vec<u32>>,
    pq: &mut BinaryHeap<Path>,
    visited_map: &mut HashMap<(i32, i32), u32>,
    curr_path: &Path,
    direction: (i32, i32),
) {
    let (c_i, c_j) = curr_path.node;
    let (i, j) = (c_i + direction.0, c_j + direction.1);
    // Invalid node
    if i < 0 || i > grid.len() as i32 - 1 || j < 0 || j > grid[0].len() as i32 - 1 {
        return;
    }

    if visited_map.contains_key(&(i, j)) && visited_map.get(&(i,j)).unwrap() < &(curr_path.cost + grid[i as usize][j as usize]){
        println!("path cost : {} vs discovered cost  {}",(curr_path.cost + grid[i as usize][j as usize]), visited_map.get(&(i,j)).unwrap() );
        print_record(&curr_path.record);
        return;
    }

    let mut block_length = curr_path.block_length;
    let mut block_direction = direction;
    if curr_path.block_direction == direction || curr_path.block_direction == (0, 0) {
        block_length += 1;
    } else {
        block_length = 1;
    }

    // continous path too long, ignore
    if block_length > 3 {
        return;
    }

    // Add node to priority queue
    let next_record  = NodeRecord {
        pos : (i,j),
        direction : block_direction,
        block_length,
    };

    let mut record_vec : Vec<NodeRecord> = curr_path.record.clone();
    record_vec.push(next_record);

    let node: Path = Path {
        node: (i, j),
        cost: curr_path.cost + grid[i as usize][j as usize],
        block_length,
        block_direction: block_direction,
        record: record_vec,
    };
    visited_map.insert((i, j),curr_path.cost + grid[i as usize][j as usize] );
    pq.push(node);
}

fn print_record(path_record : &Vec<NodeRecord>) {
    for rec in path_record {
        print!("{:?} - ", rec.pos);
    }
    println!();
}
