use std::io::prelude::*;

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open input.txt");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let lines = contents.lines();

    let mut platform : Vec<Vec<char>> = Vec::new();
    for line in lines {
        let row : Vec<char> = line.chars().collect();
        platform.push(row);
    }

    // Roll up
    let mut col_index: usize = 0;
    while col_index < platform[0].len() {

        // Iterate through column
        let mut next_empty_index : usize = usize::MAX;
        for row_index in 0..=platform.len()-1 {
            // At each index move up as much as possible 
            match platform[row_index][col_index] {
                'O' => {
                    if next_empty_index == usize::MAX {
                        continue;
                    }
                    if next_empty_index < row_index {
                        platform[next_empty_index][col_index] = 'O';
                        platform[row_index][col_index] = '.';

                        while next_empty_index < platform.len() 
                        && (platform[next_empty_index][col_index] == 'O' 
                        ||    platform[next_empty_index][col_index] == '#') 
                        {
                            next_empty_index += 1;
                        }

                        // full column, exit
                        if next_empty_index >= platform.len() {
                            break;
                        }
                    }
                },

                '#' => {
                    if next_empty_index == usize::MAX {
                        continue;
                    }

                    next_empty_index = row_index + 1;
                    if next_empty_index >= platform.len() {
                        break;
                    }

                    while next_empty_index < platform.len() 
                    &&   (platform[next_empty_index][col_index] == 'O' 
                    ||    platform[next_empty_index][col_index] == '#' )
                    {
                        next_empty_index += 1;
                    }
                    // full column, exit
                    if next_empty_index >= platform.len() {
                        break;
                    }
                },

                '.' => {
                    next_empty_index = core::cmp::min(row_index, next_empty_index);
                },

                _ => {
                    continue;
                },                
            }
        }

        col_index += 1;
    }

    // Compute load


    let mut sum : i64 = 0;
    for (i,row) in platform.iter().enumerate() {
        let o_count : i64 = row.iter()
            .filter(|&&c| c == 'O')
            .count() as i64;

        sum += o_count * (platform.len() as i64 - i as i64);
        println!("{:?}", row);
    }

    println!("SUM {}", sum);
}
