use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Lens {
    id: String,
    focal_length: i32,
}

impl Lens {
    fn new(id: &str, focal_length: i32) -> Self {
        Lens {
            id: id.to_string(),
            focal_length,
        }
    }
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file to string");

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    let hash_inputs: Vec<&str> = contents.split(',').collect();
    let mut hash_sum: i64 = 0;
    for input in hash_inputs {
        // Process = case
        if input.contains('=') {
            let split_input: Vec<&str> = input.split('=').collect();
            let id = split_input[0];
            let focal_length: i32 = split_input[1].parse::<i32>().unwrap();
            let lens = Lens::new(id, focal_length);

            let hash_index = hash(id);
            if let Some(index) = boxes[hash_index].iter().position(|l| *l == lens) {
                boxes[hash_index][index] = lens;
            }
            else {
                boxes[hash_index].push(lens);
            }
        }
        // Process - case
        else {
            let split_input: Vec<&str> = input.split('-').collect();
            let id = split_input[0];
            let lens = Lens::new(id, 1);

            let hash_index = hash(id);
            if let Some(index) = boxes[hash_index].iter().position(|l| *l == lens) {
                boxes[hash_index].remove(index);
            }
        }
    }

    let mut total_power: i64 = 0;
    for (index, box_i) in boxes.iter().enumerate() {
        if box_i.is_empty() {
            continue;
        }

        for (slot, lens) in box_i.iter().enumerate() {
            let focusing_power = (1 + index as i64) * (1 + slot as i64) * (lens.focal_length as i64);
            total_power += focusing_power;
        }
    }

    println!("Total Power : {}", total_power);
}

fn hash(input: &str) -> usize {
    let mut current_value: usize = 0;
    for c in input.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}
