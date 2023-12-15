use std::io::prelude::*;

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file to string");

    let hash_inputs: Vec<&str> = contents.split(',').collect();
    let mut hash_sum: i64 = 0;
    for input in hash_inputs {
        hash_sum += hash(input);
    }
    println!("Sum : {}", hash_sum);
}

fn hash(input: &str) -> i64 {
    let mut current_value: i64 = 0;
    for c in input.chars() {
        current_value += c as i64;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}
