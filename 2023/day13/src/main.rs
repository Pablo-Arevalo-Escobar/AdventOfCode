use std::io::prelude::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAR_TO_BINARY : HashMap<char, char> = {
        let mut char_to_binary = HashMap::new();
        char_to_binary.insert('#', '1');
        char_to_binary.insert('.', '0');
        char_to_binary
    };
}


fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let lines = contents.lines();

    let mut row_sum : i64 = 0;
    let mut col_sum : i64 = 0;

    // read data into grid, for ease of char access
    let mut grid : Vec<Vec<char>> = Vec::new();

    // Store the rows and columns as u32
    // Treating # as 1 and . as 0 we can use a binary representation for each row and col
    // We can then perform AND to check for symmetry 
    let mut rows : Vec<u32> = Vec::new();
    // read rows and make grid for reading columns
    for line in lines {
        if line.is_empty() {
            let grid_value = process_grid(&mut grid,&rows);
            row_sum += grid_value.0;
            col_sum += grid_value.1;
            
            rows.clear();
            grid.clear();
            continue;
        }
        rows.push(binary_from_line(line));
        let row : Vec<char> = line.chars().collect::<Vec<char>>(); 
        grid.push(row);
    }

    if ! rows.is_empty() {
        let grid_value = process_grid(&mut grid,&rows);
        row_sum += grid_value.0;
        col_sum += grid_value.1;
        
    }

    println!("row sum : {}", row_sum);
    println!("col sum : {}", col_sum);
    println!("total : {}", row_sum + col_sum);
}

fn process_grid(grid : &mut Vec<Vec<char>>, rows : &Vec<u32>) -> (i64, i64) {
    let mut row_sum : i64 = 0;
    let mut col_sum : i64 = 0;
    let mut columns : Vec<u32> = Vec::new();

    // Read columns
    let mut col_index: usize = 0;
    while col_index <= grid[0].len() - 1 {
        let mut col_string : String = String::new();
        for row in grid.iter_mut() {
            col_string.push_str(&row[col_index].to_string());
        }
        columns.push(binary_from_line(&col_string));
        col_index += 1;
    }

    // Check rows for symmetry
    for row_index in 1..=rows.len()-1 {
        // Perform a XOR comparison and count the ones to check if the result varies by one bit, this means there was a smudge
        let xor = rows[row_index] ^ rows[row_index-1];
        let mut xor_used : bool = false;
        if xor.count_ones() == 1 {
            xor_used = true;
        }
        if xor_used || rows[row_index] == rows[row_index-1] {
            if verify_perfect_reflection(&rows, row_index-1, row_index,xor_used) {
                row_sum += 100 * row_index as i64;
                break;
            }
        }
    }


    // Check for xor and perform transformation
    for col_index in 1..=columns.len()-1 {
        // xor can only be used once to fix the smudge
        let xor = columns[col_index] ^ columns[col_index-1];
        let mut xor_used : bool = false;
        if xor.count_ones() == 1 {
            xor_used = true;
        }
        if xor_used || columns[col_index-1] == columns[col_index] {
            if verify_perfect_reflection(&columns, col_index-1, col_index, xor_used) {
                col_sum += col_index as i64;
                break;
            }  
        }
    }

    return (row_sum,col_sum);
}

fn verify_perfect_reflection(lines : &Vec<u32>, left : usize, right : usize, mut xor_used  : bool) -> bool {
    // Verify perfect reflection
    let mut delta : usize = 1;
    let mut is_perfect : bool = true;
    while (right as i32 + delta as i32) < lines.len() as i32 && (left as i32- delta as i32) >= 0 {

        let top_row : u32 = lines[left-delta];
        let bot_row : u32 = lines[right+delta];
        let xor = top_row ^ bot_row;
        let use_xor : bool = xor.count_ones() == 1;
      

        if !(top_row == bot_row) && !use_xor  || (xor_used && use_xor)
        {
            is_perfect = false;
            break;
        }

        xor_used = xor_used || use_xor;
        delta += 1;
    }

    if !xor_used 
    {
        is_perfect = false;
    }

    return is_perfect;
}

fn binary_from_line(line :&str) -> u32 {
    let mut binary_string : String = String::new();
    for c in line.chars() {
        let bin_value = CHAR_TO_BINARY.get(&c).unwrap_or(&'!');
        binary_string.push_str(&bin_value.to_string());
    }

    match u32::from_str_radix(&binary_string, 2) {
        Ok(binary_value) => {
            return binary_value;
        }

        Err(_) => {
            println!("Unable to parse binary value");
            return 0;
        }
    }
}