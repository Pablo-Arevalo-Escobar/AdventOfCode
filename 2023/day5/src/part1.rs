use std::io::prelude::*;

fn main() {
    // Read file input
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");

    let lines : Vec<&str> = contents.lines().collect();

    // Collect the initial seed information
    let seed_split : Vec<&str> = lines[0].split(":").collect();
    let mut value_set : Vec<i64>  = seed_split[1]
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();


    // Pass the seeds through the maps
    // create an identical vector that is marked as true if the value has been mapped this round 
    let mut is_mapped : Vec<bool> = vec![false; value_set.len()];
    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue;
        }
        else if line.contains(':') 
        {
            for value in is_mapped.iter_mut() {
                *value = false;
            }
            continue;
        }

        // split into dest start, source start and range
        let line_info : Vec<&str> = line.split_whitespace().collect();
        let dest_start : i64     = line_info[0].parse::<i64>().unwrap();
        let source_start : i64   = line_info[1].parse::<i64>().unwrap();
        let range : i64          = line_info[2].parse::<i64>().unwrap();
    
        value_set.iter_mut().enumerate().for_each(|(index, num)| {
            if is_mapped[index] == false && *num >= source_start && *num  < (source_start + range) {
                *num  += dest_start - source_start;
                is_mapped[index] = true;
            }
        })
    }

    println!("answer is: {}", value_set.iter().cloned().min().unwrap());
}
