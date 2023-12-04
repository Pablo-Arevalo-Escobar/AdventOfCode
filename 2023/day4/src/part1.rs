use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Read file input 
    let mut file = std::fs::File::open("input.txt")
    .expect("Unable to open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");

    let lines = contents.lines();

    let mut sum : i32 = 0;
    for line in lines {
        println!("{}",line);
        let split_line : Vec<&str> = line.split(':').collect();
        let game_info  : Vec<&str> = split_line[1].split('|').collect();

        let winning_numbers : Vec<i32> = game_info[0]
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let player_numbers : Vec<i32> = game_info[1]
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let winning_count : i32 = player_numbers.iter()
            .filter(|&n| winning_numbers.contains(n))
            .cloned()
            .count() as i32;

        let exponent : i32 = winning_count-1;
        let base : i32 = 2;

        if exponent < 0 {
            continue;
        }

        let round_points = base.pow(exponent.try_into().unwrap());
        sum += round_points;
    }
    println!("{}", sum);

}


