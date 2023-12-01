use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    // Read from input file
    let mut file = File::open("input.txt").expect("Unable to open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");
    let input_info = contents.lines();

    // Create valid number string arrray
    let starting_chars = ['o','t','f','s','e','n'];

    // Create hashmap to map written digits to numeric values
    let mut digit_map: HashMap<String,char> = HashMap::new();
    digit_map.insert("one".to_string(), '1');
    digit_map.insert("two".to_string(), '2');
    digit_map.insert("three".to_string(), '3');
    digit_map.insert("four".to_string(), '4');
    digit_map.insert("five".to_string(), '5');
    digit_map.insert("six".to_string(), '6');
    digit_map.insert("seven".to_string(), '7');
    digit_map.insert("eight".to_string(), '8');
    digit_map.insert("nine".to_string(), '9');

    // Part 1
    let mut sum: i32 = 0;
    for line in input_info {
        let mut first_integer: char = '0';
        let mut last_integer: char = '0';
        let mut is_first_found: bool = false;

        for (index ,c) in line.char_indices() {
            if c.is_numeric() {
                if !is_first_found {
                    first_integer = c;
                    is_first_found = true;
                }
                last_integer = c;
            }

            if c.is_alphabetic() && starting_chars.contains(&c) {
                for i in 2..=4 
                {
                    let end_index = index+i;
                    if end_index >= line.len() {break;}
                    let key = &line[index..=end_index];
                    if digit_map.contains_key(key)
                    {
                        let mut numeric_value: char = '0';
                        let option = digit_map.get(key);
                        match option {
                            Some(value) => {
                                numeric_value = *value;
                            }
                            None => {}
                        }
                        if !is_first_found {
                            first_integer = numeric_value;
                            is_first_found = true;
                        }
                        last_integer = numeric_value;
                    }
                }
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
