use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Read from input file
    let mut file = File::open("input.txt").expect("Unable to open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");

    let lines: Vec<_> = contents.lines().collect();

    let mut sum: i32 = 0;
    for index in 0..=lines.len()-1{
        let line = lines[index];

        let mut number_found = false;
        let mut num_is_valid = false;
        let mut num_string = String::new();

        for line_index in 0..line.len() {
            let c = line.chars().nth(line_index).unwrap();
            //print!("{} ", c);

            // Number enountered, start validation
            if c.is_digit(10) {
                if !number_found {
                    number_found = true;
                    num_string = String::new();
                }
                num_string.push(c);
                //println!("num_string: {}", num_string);
                // Perform validation:
                if index > 0 {
                    // directly above
                    let line_above = lines[index - 1];

                    let c_dabove = line_above.chars().nth(line_index).unwrap();
                    if !c_dabove.is_numeric() && c_dabove != '.' {
                        num_is_valid = true;
                    }

                    // above and left
                    if line_index > 0 {
                        let c_labove = line_above.chars().nth(line_index - 1).unwrap();
                        if !c_labove.is_numeric() && c_labove != '.' {
                            num_is_valid = true;
                        }
                    }
                }

                if index < lines.len() - 1 {
                    let line_below = lines[index + 1];

                    // Check directly below
                    let c_dbelow = line_below.chars().nth(line_index).unwrap();
                    if !c_dbelow.is_numeric() && c_dbelow != '.' {
                        num_is_valid = true;
                    }

                    // Check below and left
                    if line_index > 0 {
                        let c_lbelow = line_below.chars().nth(line_index - 1).unwrap();
                        if !c_lbelow.is_numeric() && c_lbelow != '.' {
                            num_is_valid = true;
                        }
                    }
                }

                // Check left for symbol
                if line_index > 0 {
                    let c_left = lines[index].chars().nth(line_index - 1).unwrap();
                    if !c_left.is_numeric() && c_left != '.' {
                        num_is_valid = true;
                    }
                }
            }


            // Number validation finished
            else if number_found {

                // Perform validation:
                if index > 0 {
                    // directly above
                    let line_above = lines[index - 1];

                    let c_dabove = line_above.chars().nth(line_index).unwrap();
                    if !c_dabove.is_numeric() && c_dabove != '.' {
                        num_is_valid = true;
                    }
                    // above and left
                    if line_index > 0 {
                        let c_labove = line_above.chars().nth(line_index - 1).unwrap();
                        if !c_labove.is_numeric() && c_labove != '.' {
                            num_is_valid = true;
                        }
                    }
                }

                if index < lines.len() - 1 {
                    let line_below = lines[index + 1];
                     // Check directly below
                     let c_dbelow = line_below.chars().nth(line_index).unwrap();
                     if !c_dbelow.is_numeric() && c_dbelow != '.' {
                         num_is_valid = true;
                     }
                    // Check below and left
                    if line_index > 0 {
                        let c_lbelow = line_below.chars().nth(line_index - 1).unwrap();
                        if !c_lbelow.is_numeric() && c_lbelow != '.' {
                            num_is_valid = true;
                        }
                    }
                }

                // Check left for symbol
                if line_index > 0 {
                    let c_left = lines[index].chars().nth(line_index - 1).unwrap();
                    if !c_left.is_numeric() && c_left != '.' {
                        num_is_valid = true;
                    }
                }
                
                if !c.is_numeric() && c != '.' {
                    num_is_valid = true;
                }

                number_found = false;
                // print if the number is valid
                if num_is_valid {
                    //println!("Valid number: {}", num_string);
                    let parsed_value: Result<i32,_> = num_string.parse();
                    match parsed_value {
                        Ok(value) => {
                            sum += value;
                        },
                        Err(e) => {
                            println!("Invalid number: {}", e);
                        }
                    }
                }

                num_is_valid = false;
            }
        }

        if number_found {
            number_found = false;
            // print if the number is valid
            if num_is_valid {
                //println!("Valid number: {}", num_string);
                let parsed_value: Result<i32,_> = num_string.parse();
                match parsed_value {
                    Ok(value) => {
                        sum += value;
                    },
                    Err(e) => {
                        println!("Invalid number: {}", e);
                    }
                }
            }
            num_is_valid = false;
        }
    }

    println!("Sum: {}", sum);
}
