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

    let times : Vec<i32>  = time_split[1]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let distance_record : Vec<i32> = distance_split[1]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("times {:?}" , times);
    println!("distances {:?}" , distance_record);

    // Perform the algorithm
    let mut ways_to_beat_record : i32 = 1;
    for i in 0..= times.len()-1 {
        let mut way_to_win_round : i32 = 0;
        let race_time : i32  = times[i];
        let race_time_half : usize = ((race_time as f32) * 0.5) as usize;
        for j in 1..= race_time_half {
            let hold_time : i32 = j as i32;
            let time_left : i32 = race_time - hold_time;
            let distance_covered = hold_time * time_left;

            if distance_covered > distance_record[i] {
                //println!(" hold time {} time left {}", hold_time, time_left);
                //println!(" {} ", (hold_time*(time_left)));
                if j as f32 == ((race_time as f32) * 0.5) {
                    way_to_win_round += 1;
                }
                else {
                    way_to_win_round += 2;
                }
            }            
        }
        println!("ways_to_win {} {}", times[i], way_to_win_round);
        ways_to_beat_record *= way_to_win_round;
        way_to_win_round = 0;
    }
    println!("ways_to_win total {}", ways_to_beat_record);

}
