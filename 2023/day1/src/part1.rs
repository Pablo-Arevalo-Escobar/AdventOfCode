use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Read from input file
    let mut file = File::open("input.txt").expect("Unable to open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
    .expect("Unable to read file!");
    let input_info = contents.lines();

    // Part 1
    let mut sum: i32 = 0;
    for line in input_info {
        let mut first_integer: char = '0';
        let mut last_integer: char = '0';
        let mut is_first_found: bool = false;

        for char in line.chars() {
            if char.is_numeric() {
                if !is_first_found {
                    first_integer = char;
                    is_first_found = true;
                }
                last_integer = char;
            }
        }

        let line_value_result: Result<i32, _> =
            format!("{}{}", first_integer, last_integer).parse();

        match line_value_result {
            Ok(number) => {
                sum += number;
            }
            Err(err) => {
                println!("Error parsing {}", err);
            }
        }
    }
    println!("{}", sum);
}
