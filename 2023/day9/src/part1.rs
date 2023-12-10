use std::io::prelude::*;

fn main() {
    // Read file input 
    let mut file = std::fs::File::open("input.txt")
        .expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let lines = contents.lines();

    // Initialize freq map
    let mut sum : i64 = 0;
    for line in lines {
        let history : Vec<i64> = line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        println!("{:?}", history);
        let mut next_value = history[history.len()-1];
        println!("next value before {}", next_value);
        next_value += get_history_value(history);
        println!("next value after {}", next_value);
        sum += next_value;
    }
    println!("sum {}", sum);
}


fn get_history_value(history : Vec<i64>) -> i64 {
    let mut difference_vector : Vec<i64> = Vec::new();
    let mut is_zero_vec : bool = true;
    for i in 0..=history.len()-2 {
        let difference = history[i+1] - history[i];
        difference_vector.push(difference);
        if difference != 0 {
            is_zero_vec = false;
        }
    }

    // Zero vector
    let mut dif_debug: Vec<i64> = difference_vector.clone();
    if is_zero_vec {
        dif_debug.push(0);
        println!("{:?}", dif_debug);
        return 0;
    }

    let last_difference : i64 =difference_vector[difference_vector.len()-1];
    let difference_increase = get_history_value(difference_vector);
    dif_debug.push(last_difference + difference_increase);
    println!("{:?}", dif_debug);
    return last_difference  + difference_increase;
}
