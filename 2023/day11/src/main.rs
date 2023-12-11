use std::collections::HashMap;
use std::io::prelude::*;

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let lines = contents.lines();

    // Step 1: Read input and format (expand space and store pair locations)
    let mut universe: Vec<Vec<String>> = Vec::new();
    let mut galaxy_count: u32 = 1;

    for line in lines {
        let row_chars: Vec<char> = line.chars().collect();
        let mut row: Vec<String> = row_chars.into_iter().map(|c| c.to_string()).collect();
        let mut is_empty_row = true;
        for s in row.iter_mut() {
            if *s == "#" {
                is_empty_row = false;
                *s = galaxy_count.to_string();
                galaxy_count += 1;
            }
        }

        // expand empty space (row expansion)
        if is_empty_row {
            for s in row.iter_mut() {
                *s = "M".to_string();
            }
        }

        universe.push(row);
    }

    // column expansion
    let mut col_index: usize = 0;
    while col_index < universe[0].len() - 1 {
        //println!("col index {}", col_index);

        let is_empty_col = universe
            .iter()
            .all(|row| row[col_index] == "." || row[col_index] == "M");

        // skip if not empty
        if !is_empty_col {
            col_index += 1;
            continue;
        }

        for row in universe.iter_mut() {
            row[col_index] = "M".to_string();
            // row.insert(col_index, ".".to_string());
        }
        //println!("col {} is empty : {}", col_index, is_empty_col);
        col_index += 2;
    }

    // for row in &universe {
    //     println!("{:?}", row);
    // }

    // hashmap initialization
    let mut galaxy_map: HashMap<u32, (usize, usize)> = HashMap::new();
    for (i, row) in universe.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if value != "." && value != "M" {
                let id: u32 = value.parse::<u32>().expect("unable to parse value");
                galaxy_map.insert(id, (i, j));
            }
        }
    }

    // Step 2: Sum shortest paths between pairs
    // 1. find all pairs
    // 2. compute manhattan distance
    let mut sum: u64 = 0;
    for i in 1..=galaxy_count - 1 {
        for j in i + 1..=galaxy_count - 1 {
            //println!("Count Pair : {} {}", i, j);
            let i_loc = galaxy_map[&i];
            let j_loc = galaxy_map[&j];
            //println!("i loc {:?}", i_loc);
            //println!("j loc {:?}", j_loc);
            let start = (i_loc.0 as i32, i_loc.1 as i32);
            let end = (j_loc.0 as i32, j_loc.1 as i32);
            let man_dist: u64 = manhattan_distance(&universe, start, end);
            sum += man_dist;
            //println!("manhattan distance {}", man_dist);
        }
    }
    println!("sum {}", sum);
}

fn manhattan_distance(grid: &Vec<Vec<String>>, start: (i32, i32), end: (i32, i32)) -> u64 {
    // traverse grid and consider M
    //println!("start {:?}", start);
    //println!("end {:?}", end);
    const EXPANSION : i64 = 1000000;
    let mut sum: u64 = 0;

    // Traverse col
    let mut col_index: i32 = start.0;
    let mut end_index: i32 = end.0;
    if start.0 > end.0 {
        col_index = end.0;
        end_index = start.0;
    }
    let mut step_count: i64 = 0;
    while col_index < end_index {
        //println!("col {} row {}", start.1, col_index);
        if grid[col_index as usize][start.1 as usize] == "M" {
            step_count += EXPANSION;
        } else {
            step_count += 1;
        }
        col_index += 1;
        //println!("step count {}", step_count);
    }

    sum += step_count as u64;

    // Traverse row
    //println!("traverse row");
    let mut row_index: i32 = start.1;
    let mut end_index: i32 = end.1;
    if start.1 > end.1 {
        row_index = end.1;
        end_index = start.1;
    } 
    step_count = 0;
    while row_index < end_index {
        //println!("row {} col {}",start.0,row_index);
        if grid[start.0 as usize][row_index as usize] == "M" {
            step_count += EXPANSION;
        } else {
            step_count += 1;
        }
        row_index += 1;
        //println!("step count {}", step_count);
    }

    sum += step_count as u64;
    return sum;
    // Traverse row
}
