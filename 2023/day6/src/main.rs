use std::collections::HashMap;
use std::io::prelude::*;

fn main() {
    // Read file input
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");

    let lines : Vec<&str> = contents.lines().collect();

    // Read file info into a time and distance vec
    let time_split : Vec<&str> = lines[0].split(':').collect();
    let distance_split : Vec<&str> = lines[1].split(':').collect();

    let race_time : i64  = time_split[1]
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();
    

    let distance_record : i64 = distance_split[1]
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    println!("times {}" , race_time);
    println!("distances {}" , distance_record);

    let mut ways_to_win : i64 = 0;
    let race_time_half : usize = ((race_time as f32) * 0.5) as usize;
    for j in 1..= race_time_half {
        let hold_time : i64 = j as i64;
        let time_left : i64 = race_time - hold_time;
        let distance_covered = hold_time * time_left;
        if distance_covered > distance_record {
            //println!(" hold time {} time left {}", hold_time, time_left);
            //println!(" {} ", (hold_time*(time_left)));
            if j as f64 == ((race_time as f64) * 0.5) {
                ways_to_win += 1;
            }
            else {
                ways_to_win += 2;
            }
        }            
    }
    println!("ways_to_win total {}", ways_to_win);

}
