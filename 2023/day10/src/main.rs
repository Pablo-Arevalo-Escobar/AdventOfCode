use std::io::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::thread::Builder;
use core::cmp::max;

lazy_static! {
    // values that can be accessed by moving up
    static ref top_set: HashSet<char> = {
        let mut set = HashSet::new();
        set.insert('|');
        set.insert('F');
        set.insert('7');
        set.insert('S');
        set
    };

    // values that can be accessed by moving down
    static ref bot_set: HashSet<char> = {
        let mut set = HashSet::new();
        set.insert('|');
        set.insert('L');
        set.insert('J');
        set.insert('S');
        set
    };

    // values that can be accessed by moving to the left
    static ref left_set: HashSet<char> = {
        let mut set = HashSet::new();
        set.insert('L');
        set.insert('-');
        set.insert('F');
        set.insert('S');
        set
    };

    // values that can be accessed by moving right
    static ref right_set: HashSet<char> = {
        let mut set = HashSet::new();
        set.insert('7');
        set.insert('-');
        set.insert('J');
        set.insert('S');
        set
    };
}


fn main() {
    let thread = Builder::new()
    .stack_size(50 * 1024 * 1024) // Set the stack size to 4 MB
    .spawn(|| {
        part_one();
    });

    if let Err(e) = thread {
        eprintln!("Error spawning thread: {:?}", e);
    } else {
        thread.unwrap().join().unwrap();
    }
}

fn part_one() {
     // Read file input
     let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
     let mut contents = String::new();
     file.read_to_string(&mut contents)
         .expect("Unable to read file!");
     let lines = contents.lines();
 
     let mut grid : Vec<Vec<char>> = Vec::new();
     let mut start_i : usize = usize::MAX;
     let mut start_j : usize = usize::MAX;
     for (i,line) in lines.enumerate() {
         let mut row : Vec<char> = line.chars().collect();
 
         // Store starting point for dfs
         if let Some(index) = row.iter().position(|&x| x == 'S') {
             start_i = i ;
             start_j = index;
         }
 
         grid.push(row);
     }
 
     for row in &grid {
        // println!("{:?}", row);
     }
 
     let mut visited_set : HashSet<String> = HashSet::new();
     let start_index = start_i.to_string() + &start_j.to_string();
     visited_set.insert(start_index);
 
     let mut left_steps : i32 = 0;
     let mut right_steps : i32 = 0;
     let mut top_steps : i32 = 0;
     let mut bot_steps : i32 = 0;

     println!("STARTING VALUE {} ", grid[start_i][start_j]);
 
     if start_j > 0 && left_set.contains(&grid[start_i][start_j-1]) {
        let left_index = start_i.to_string() + &(start_j-1).to_string();
        let mut new_set = visited_set.clone();
        new_set.insert(left_index);
        left_steps = dfs(&grid,new_set, 0, start_i, start_j-1);
     }
     if start_j < grid.len()-1  && right_set.contains(&grid[start_i][start_j+1]){
        let right_index = start_i.to_string() + &(start_j+1).to_string();
        let mut new_set = visited_set.clone();
        new_set.insert(right_index);
        right_steps = dfs(&grid,new_set, 0, start_i, start_j+1);
     }
    if start_i > 0 && top_set.contains(&grid[start_i-1][start_j]) {
        let top_index = (start_i-1).to_string() + &(start_j).to_string();
        let mut new_set = visited_set.clone();
        new_set.insert(top_index);
        top_steps = dfs(&grid,visited_set.clone(), 0, start_i-1, start_j);
    }
     if start_i < grid[0].len()-1 && bot_set.contains(&grid[start_i+1][start_j]) {
        let bot_index = (start_i+1).to_string() + &start_j.to_string();
        let mut new_set = visited_set.clone();
        new_set.insert(bot_index);
        bot_steps = dfs(&grid,visited_set.clone(), 0, start_i+1, start_j);
     }
 
     let max_steps = max(left_steps, max(right_steps, max(top_steps, bot_steps)));
     println!("{}",max_steps/2);
 
     //println!("{}", right_steps/2);
}

fn dfs(grid : &Vec<Vec<char>>, mut visited_set : HashSet<String>, steps: i32, i : usize, j : usize) -> i32 {
    if i < 0 || i > grid.len()-1 || j < 0 || j > grid[0].len()-1 {
        return -1;
    }

    let curr_node = grid[i][j];
    println!("curr_node {}", curr_node);
    println!("steps {}", steps);

    let mut left_node  = '.';
    let mut right_node = '.';
    if j > 0 {
        left_node = grid[i][j-1];
    }
    if j < grid[0].len()-1 {
        right_node = grid[i][j+1];
    }

    let mut top_node = '.';
    let mut bot_node = '.';
    if i > 0 {
        top_node = grid[i-1][j];
    }
    if i < grid.len()-1 
    {
        bot_node = grid[i+1][j];
    }


    // traversal logic
    let mut left_dfs : i32 = -1;
    let mut right_dfs : i32 = -1;
    let mut bot_dfs : i32 = -1;
    let mut top_dfs : i32 = -1;
    match curr_node {
    '-' => {
        let left_index = i.to_string() + &(j as i32 -1).to_string();
        if left_set.contains(&left_node) && !visited_set.contains(&left_index) || left_node == 'S' && steps > 1 {
            visited_set.insert(left_index);
            left_dfs = dfs(&grid,visited_set.clone(),steps+1,i,j-1);
        }

        let right_index = i.to_string() + &(j as i32 +1).to_string();
        if right_set.contains(&right_node) && !visited_set.contains(&right_index) || right_node == 'S' && steps > 1 {
            visited_set.insert(right_index);
            right_dfs = dfs(&grid,visited_set.clone(), steps+1, i, j+1);
        }
    },
    
    '|' => {
        let top_index = (i as i32 -1).to_string() + &j.to_string();
        if top_set.contains(&top_node)  && !visited_set.contains(&top_index) || top_node == 'S' && steps > 1 {
            visited_set.insert(top_index);
            top_dfs = dfs(&grid,visited_set.clone(), steps+1,i-1,j);
        }

        let bot_index = (i as i32 +1).to_string() + &j.to_string();
        if bot_set.contains(&bot_node) && !visited_set.contains(&bot_index) || bot_node == 'S' && steps > 1 {
            visited_set.insert(bot_index);
            bot_dfs = dfs(&grid, visited_set.clone(),steps+1, i+1, j);
        }
    },

    'L' => {
        let top_index = (i as i32 -1).to_string() + &j.to_string();
        if top_set.contains(&top_node)  && !visited_set.contains(&top_index) || top_node == 'S' && steps > 1 {
            visited_set.insert(top_index);
            top_dfs = dfs(&grid,visited_set.clone(), steps+1,i-1,j);
        }


        let right_index = i.to_string() + &(j as i32 +1).to_string();
        if right_set.contains(&right_node) && !visited_set.contains(&right_index) || right_node == 'S' && steps > 1 {
            visited_set.insert(right_index);
            right_dfs = dfs(&grid,visited_set.clone(), steps+1, i, j+1);
        }
    },

    'J' => {
        let top_index = (i as i32 -1).to_string() + &j.to_string();
        if top_set.contains(&top_node)  && !visited_set.contains(&top_index) || top_node == 'S' && steps > 1 {
            visited_set.insert(top_index);
            top_dfs = dfs(&grid,visited_set.clone(), steps+1,i-1,j);
        }
        
        let left_index = i.to_string() + &(j as i32 -1).to_string();
        if left_set.contains(&left_node) && !visited_set.contains(&left_index)  || left_node == 'S' && steps > 1 {
            visited_set.insert(left_index);
            left_dfs = dfs(&grid,visited_set.clone(),steps+1,i,j-1);
        }
    },
    '7' => {
        let bot_index = (i as i32 +1).to_string() + &j.to_string();
        if bot_set.contains(&bot_node) && !visited_set.contains(&bot_index) || bot_node == 'S' && steps > 1 {
            visited_set.insert(bot_index);
            bot_dfs = dfs(&grid, visited_set.clone(),steps+1, i+1, j);
        }
        
        let left_index = i.to_string() + &(j as i32 -1).to_string();
        if left_set.contains(&left_node) && !visited_set.contains(&left_index) || left_node == 'S' && steps > 1 {
            visited_set.insert(left_index);
            left_dfs = dfs(&grid,visited_set.clone(),steps+1,i,j-1);
        }
    },
    'F' =>  {
        let bot_index = (i as i32 +1).to_string() + &j.to_string();
        if bot_set.contains(&bot_node) && !visited_set.contains(&bot_index) || bot_node == 'S' && steps > 1 {
            visited_set.insert(bot_index);
            bot_dfs = dfs(&grid, visited_set.clone(),steps+1, i+1, j);
        }
        
        let right_index = i.to_string() + &(j as i32 +1).to_string();
        if right_set.contains(&right_node) && !visited_set.contains(&right_index) || right_node == 'S' && steps > 1 {
            visited_set.insert(right_index);
            right_dfs = dfs(&grid,visited_set.clone(), steps+1, i, j+1);
        }
    },
    'S' => return steps+1,
    '.' => return -1,
     _ => return -1,
    }

    return max(bot_dfs, max(top_dfs, max(left_dfs, right_dfs)));
}
