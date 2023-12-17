use std::collections::HashSet;
use std::io::Read;
use std::collections::VecDeque;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Move {
    cell: (i32, i32),
    direction: (i32, i32),
}

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read file");

    let mut mirror_set: HashSet<Move> = HashSet::new();
    let mut visited_set: HashSet<(i32, i32)> = HashSet::new();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in content.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    for row in grid.iter_mut() {
        println!("{:?}", row);
    }

    // Start moving the light in direction, apply modifiers and save split records
    let start_direction: (i32, i32) = (0, 1);
    let start_cell: (i32, i32) = (0, 0);
    let mut move_queue: VecDeque<Move> = VecDeque::new();

    let first_move: Move = Move {
        cell: start_cell,
        direction: start_direction,
    };

    move_queue.push_back(first_move);
    // movement loop
    while !move_queue.is_empty() {
        //println!("move_stack {:?}", move_stack);
        let current_move: Move = move_queue.pop_front().unwrap();
        let (cell_i, cell_j) = current_move.cell;
        let (dir_i, dir_j) = current_move.direction;

        // move is out of range
        if cell_i < 0 || cell_i >= grid.len() as i32 || cell_j < 0 || cell_j >= grid[0].len() as i32
        {
            continue;
        }

        // check if previously visited 
        if mirror_set.contains(&current_move) {
            continue;
        }

        //println!("Current Move : {:?}", current_move);
        // Update visited set
        visited_set.insert(current_move.cell);
        // for i in 0..=grid.len()-1 {
        //     for j in 0..=grid[0].len()-1 {
        //         if visited_set.contains(&(i as i32, j as i32)) {
        //             print!(" # ");
        //         }
        //         else {
        //             print!(" . ");
        //         }
        //     }
        //     println!();
        // }
        

        let mirror: char = grid[cell_i as usize][cell_j as usize];
        let mut direction: (i32, i32) = current_move.direction;
        match mirror {
            '-' if direction.1 == 0 => {
                mirror_set.insert(current_move);
                // split light into horizontal directions
                let left_split = (cell_i, cell_j - 1);
                let right_split = (cell_i, cell_j + 1);

                let left_move: Move = Move {
                    cell: left_split,
                    direction: (0, -1),
                };
                let right_move: Move = Move {
                    cell: right_split,
                    direction: (0, 1),
                };

                move_queue.push_back(left_move);
                move_queue.push_back(right_move);
            }

            '|' if direction.0 == 0 => {
                mirror_set.insert(current_move);
                // split light into vertical directions
                let top_split = (cell_i - 1, cell_j);
                let bot_split = (cell_i + 1, cell_j);

                let top_move: Move = Move {
                    cell: top_split,
                    direction: (-1, 0),
                };
                let bot_move: Move = Move {
                    cell: bot_split,
                    direction: (1, 0),
                };

                move_queue.push_back(top_move);
                move_queue.push_back(bot_move);
            }

            '/' => {
                mirror_set.insert(current_move);

                // coming from left/right
                if direction.0 == 0 {
                    direction.0 = direction.1 * -1;
                    direction.1 = 0;
                }
                // coming from top/bot
                else {
                    direction.1 = direction.0 * -1;
                    direction.0 = 0;
                }

                let next_cell = (cell_i + direction.0, cell_j + direction.1);
                let next_move = Move {
                    cell: next_cell,
                    direction: direction,
                };

                move_queue.push_back(next_move);
            }


            '\\' => {
                mirror_set.insert(current_move);

                // neg ( Change X and Y)
                // coming from left/right
                if direction.0 == 0 {
                    direction.0 = direction.1 * 1;
                    direction.1 = 0;
                } else {
                    direction.1 = direction.0 * 1;
                    direction.0 = 0;
                }

                let next_cell = (cell_i + direction.0, cell_j + direction.1);
                let next_move = Move {
                    cell: next_cell,
                    direction: direction,
                };
                move_queue.push_back(next_move);
            }

            _ => {
                let next_move: Move = Move {
                    cell: (cell_i + dir_i, cell_j + dir_j),
                    direction: current_move.direction,
                };
                move_queue.push_back(next_move);
            }
        }
        println!("visited set size {}", visited_set.len());
    }



}
