use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    // Read from input file
    let mut file = File::open("input.txt")
        .expect("Unable to open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");
    
    let input_info = contents.lines();

    let mut color_map : HashMap<String,i32> = HashMap::from([
        ("red".to_string(),0),
        ("green".to_string(),0),
        ("blue".to_string(),0)
    ]);

    let color_arr = ["red".to_string(), "green".to_string(), "blue".to_string()];

    for color in color_arr {
        *color_map.entry(color).or_insert(0) += 1;
    }

    for (key, value) in &color_map {
        println!("{}: {}", key, value);
    }

    let mut sum_powers = 0;
    for line in input_info {
        let line_split: Vec<&str> = line.split(':').collect();

        // Retrieve game identifier
        let game: Vec<&str> = line_split[0].split(' ').collect();
        let game_id: &str = game[1];

        // Compute validity
        let game_outcome_split: Vec<&str> = line_split[1].split(';').collect();

        // Roll results
        for round_outcome in game_outcome_split {

            let round_outcome_split: Vec<&str> = round_outcome.trim().split(',').collect();
            println!("Outcome: {}", round_outcome.to_string());

            for roll_result in round_outcome_split  {
                let roll_result_split: Vec<&str> = roll_result.trim().split(' ').collect();
                let roll_value = roll_result_split[0].trim();
                let roll_color = roll_result_split[1].trim();
                
                match roll_value.parse::<i32>() {
                    Ok(value) => {
                        let prev_max_value = color_map.get(roll_color).unwrap_or(&0);
                        let max_value = value.max(*prev_max_value);
                        color_map.insert(roll_color.to_string(), max_value);
                    }
                    Err(_) => {
                        println!("Invalid value: {}", roll_value);
                    }
                }
            }
        }

        // Compute power of set
        let red_value = color_map.get("red").unwrap_or(&0);
        let green_value = color_map.get("green").unwrap_or(&0);
        let blue_value = color_map.get("blue").unwrap_or(&0);
        let power_of_set = red_value * green_value * blue_value;
        sum_powers += power_of_set;
        println!("Power of set: {}", power_of_set);

        // Reset frequencies
        color_map.insert("red".to_string(), 0);
        color_map.insert("green".to_string(), 0);
        color_map.insert("blue".to_string(), 0);
    }

    println!("Sum Power of Set: {}", sum_powers);
}
