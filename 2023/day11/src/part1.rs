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
        for s in row.iter_mut() {
            if *s == "#" {
                *s = galaxy_count.to_string();
                galaxy_count += 1;
            }
        }
        let is_empty_row = !row.iter().any(|c| c != ".");

        universe.push(row.clone());

        // expand empty space (row expansion)
        if is_empty_row {
            universe.push(row);
        }
    }

    // column expansion
    let mut col_index: usize = 0;
    while col_index < universe[0].len() - 1 {
        //println!("col index {}", col_index);

        let is_empty_col = universe.iter().all(|row| row[col_index] == ".");

        // skip if not empty
        if !is_empty_col {
            col_index += 1;
            continue;
        }

        for row in universe.iter_mut() {
            row.insert(col_index, ".".to_string());
        }
        col_index += 2;
        //println!("col {} is empty : {}", col_index, is_empty_col);
    }

    // hashmap initialization
    let mut galaxy_map: HashMap<u32, (usize, usize)> = HashMap::new();
    for (i, row) in universe.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if value != "." {
                let id: u32 = value.parse::<u32>().expect("unable to parse value");
                galaxy_map.insert(id, (i, j));
            }
        }
    }

    // Step 2: Sum shortest paths between pairs
    // 1. find all pairs
    // 2. compute manhattan distance
    let mut sum: u32 = 0;
    for i in 1..=galaxy_count - 1 {
        for j in i + 1..=galaxy_count - 1 {
            //println!("Count Pair : {} {}", i, j);
            let i_loc = galaxy_map[&i];
            let j_loc = galaxy_map[&j];
            //println!("i loc {:?}", i_loc);
            //println!("j loc {:?}", j_loc);

            let manhattan_distance: u32 = (i_loc.0 as i32 - j_loc.0 as i32).abs() as u32
                + (i_loc.1 as i32 - j_loc.1 as i32).abs() as u32;
            sum += manhattan_distance;
            //println!("manhattan distance {}", manhattan_distance);
        }
    }
    println!("sum {}", sum);
}
