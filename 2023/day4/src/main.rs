use std::collections::HashMap;
use std::io::prelude::*;

fn main() {
    // Read file input
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");

    let lines = contents.lines();

    // Initialize freq map
    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    let mut sum: i32 = 0;
    let mut index: i32 = 1;

    for line in lines {
        let split_line: Vec<&str> = line.split(':').collect();
        let game_info: Vec<&str> = split_line[1].split('|').collect();

        let winning_numbers: Vec<i32> = game_info[0]
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let player_numbers: Vec<i32> = game_info[1]
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let winning_count: i32 = player_numbers
            .iter()
            .filter(|&n| winning_numbers.contains(n))
            .cloned()
            .count() as i32;

        let index_card_count = *freq_map.entry(index).or_insert(1);
        sum += index_card_count;
        index += 1;
        for num in index..(index + winning_count) {
            let counter = freq_map.entry(num).or_insert(1);
            *counter += 1 * index_card_count;
        }
    }
    println!("{}", sum);
}
