use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: &str, right: &str) -> Self {
        Node {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

fn main() {
    let test_node = Node {
        left : "AAA".to_string(),
        right : "BBB".to_string(),
    };
    println!("{:?}", test_node);

    // Read file input
    let mut file = std::fs::File::open("input.txt")
        .expect("Unable to open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file!");

    let lines : Vec<&str> = contents.lines().collect();
    let mut node_map : HashMap<String, Node> = HashMap::new();
    let navigation = lines[0];

    // Part 2 
    let mut starting_nodes : Vec<&str> = Vec::new();

    // Read in map data
    for i in 1..= (lines.len()-1) as usize {
        if lines[i].is_empty() {
            continue;
        }

        let split_line : Vec<&str> = lines[i].split('=').collect();
        let node_name = split_line[0].trim();
        let left_right_data = split_line[1].replace("(","").replace(")","");
        let pair : Vec<&str> = left_right_data.split(',').collect();

        if node_name.chars().nth(2).unwrap_or('!') == 'A' {
            starting_nodes.push(node_name);
        }
        
        let node_data = Node {
            left: pair[0].trim().to_string(),
            right: pair[1].trim().to_string(),
        };
        node_map.insert(node_name.to_string(), node_data);
    }
    println!("{:?}",starting_nodes);

    
    // Start at AAA and follow navigation until ZZZ is reached.
    let mut nav_index : usize = 0;
    let mut num_of_steps : i32 = 0;
    let mut is_end : bool = false;
    while !is_end {
        is_end = true;
        let direction : char = navigation.chars().nth(nav_index).unwrap_or('L');

        //println!("{:?}", starting_nodes);
        for node_name in starting_nodes.iter_mut() {
            let node_data = node_map.get(*node_name).unwrap();
            if direction == 'L' {
                *node_name = &node_data.left;
            }
            else {
                *node_name = &node_data.right;
            }

            if node_name.chars().nth(2).unwrap() != 'Z' 
            {
                is_end = false;
            }
        }
       
        nav_index = (nav_index+1)%navigation.len();
        num_of_steps += 1;
    }
    println!("num of steps : {}", num_of_steps);



}
