use std::io::*;
use std::collections::VecDeque;
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref ARROW_SET : HashSet<char> = {
        let mut set = HashSet::new();
        set.insert('>');
        set.insert('<');
        set.insert('V');
        set.insert('^');
        set
    };
} 
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone )]
struct Traversal {
    visited_set : HashSet<(i32, i32)>,
    position : (i32,i32),
}
fn main() {
    let mut file = std::fs::File::open("input.txt")
        .expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");


    let mut grid : Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let row : Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let mut queue : VecDeque<Traversal> = VecDeque::new();
    let target = ((grid.len() as i32) -1,(grid[0].len() as i32) -2);
    let start_traversal = Traversal{
        visited_set: HashSet::new(),
        position : (0,1),
    };

    queue.push_back(start_traversal);

    let mut max_path : i32 = 0;
    while !queue.is_empty() {
        let mut traversal = queue.pop_front().unwrap();
        let pos = traversal.position;

        if !is_in_range(&grid, pos) || traversal.visited_set.contains(&traversal.position) {
            continue;
        }

        traversal.visited_set.insert(traversal.position);
        println!("{:?}", pos);

        if pos == target {
            visualize(&grid, &traversal);
            println!("TARGET REACHED {}", traversal.visited_set.len() -1);
            max_path = std::cmp::max(max_path, traversal.visited_set.len() as i32);
            continue;
        }

        

        match grid[pos.0 as usize][pos.1 as usize] {
            '>' => {
                queue.push_back(make_traversal(&grid, Direction::Right, &traversal)); 
            },
            '<' => {
                queue.push_back(make_traversal(&grid, Direction::Left, &traversal));
            },
            'V' | 'v' => {
                queue.push_back(make_traversal(&grid, Direction::Down, &traversal));
            },
            '^' => {
                queue.push_back(make_traversal(&grid, Direction::Up, &traversal));  
            },
            _ => {
                queue.push_back(make_traversal(&grid, Direction::Up, &traversal)); 
                queue.push_back(make_traversal(&grid, Direction::Down, &traversal));
                queue.push_back(make_traversal(&grid, Direction::Right, &traversal)); 
                queue.push_back(make_traversal(&grid, Direction::Left, &traversal));
            },
        }
 
        
    }

    println!("Target : {:?}", target);
    println!("MAX {}", max_path - 1);

}

fn make_traversal(grid: &Vec<Vec<char>> , direction : Direction, traversal : &Traversal) -> Traversal {
    let mut target_pos: (i32,i32) = (0,0);
    let pos = traversal.position;
    match direction {
        Direction::Left => {
            target_pos = (pos.0, pos.1 -1);
        },
        Direction::Right => {
            target_pos = (pos.0, pos.1 + 1);
        },
        Direction::Up => {
            target_pos = (pos.0 -1, pos.1);
        },
        Direction::Down => {
            target_pos = (pos.0 +1, pos.1);
        },
    }

    let mut new_traversal = traversal.clone();
    new_traversal.position = (-1,-1);
    if is_valid(&grid, target_pos) && !traversal.visited_set.contains(&target_pos) {
        new_traversal.position = target_pos;
    }
    new_traversal
}

fn is_valid(grid: &Vec<Vec<char>>, pos : (i32,i32)) -> bool {
    if !is_in_range(grid, pos) || grid[pos.0 as usize][pos.1 as usize] == '#' {
        return false;
    }
    return true;
}

fn is_in_range(grid : &Vec<Vec<char>>, pos : (i32, i32)) -> bool {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= grid.len() as i32 || pos.1 >= grid[0].len() as i32 {
        return false;
    }
    return true;
}

fn visualize(grid : &Vec<Vec<char>>, traversal : &Traversal) {
    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if traversal.visited_set.contains(&(i as i32, j as i32)) {
                print!("O ");
            }
            else {
                print!("{} ", val);
            }
        }
        println!();
    }

    for _n in 0..=10 {
        println!();
    }
}
