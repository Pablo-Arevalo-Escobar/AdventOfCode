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

    let color_map : HashMap<String,i32> = HashMap::from([
        ("red".to_string(),12),
        ("green".to_string(),13),
        ("blue".to_string(),14)
    ]);

    for (key,value) in color_map.iter() {
        let color_value = color_map.get(key).unwrap_or(&0);
        println!("Key: {}, Value: {}", key, color_value);
    }


    let mut sum_game_id = 0;
    for line in input_info {
        let line_split: Vec<&str> = line.split(':').collect();

        // Retrieve game identifier
        let game: Vec<&str> = line_split[0].split(' ').collect();
        let game_id: &str = game[1];

        // Compute validity
        let mut is_valid: bool = true;
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
                        if value > *color_map.get(roll_color).unwrap_or(&0) {
                            is_valid = false;
                            break;
                        }
                        println!("Value: {}", value);
                    }
                    Err(_) => {
                        is_valid = false;
                        break;
                    }
                }
            }

            // Break if not valid
            if !is_valid {
                break;
            }
        }

        if is_valid {
            sum_game_id += game_id.parse::<i32>().unwrap_or(0);
        }
    }

    println!("Sum Game ID: {}", sum_game_id);
}
