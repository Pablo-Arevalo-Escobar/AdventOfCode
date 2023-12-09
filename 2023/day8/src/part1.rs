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

    // Read in map data
    for i in 1..= (lines.len()-1) as usize {
        if lines[i].is_empty() {
            continue;
        }

        let split_line : Vec<&str> = lines[i].split('=').collect();
        let node_name = split_line[0];
        let left_right_data = split_line[1].replace("(","").replace(")","");
        let pair : Vec<&str> = left_right_data.split(',').collect();
        
        let node_data = Node {
            left: pair[0].trim().to_string(),
            right: pair[1].trim().to_string(),
        };
        node_map.insert(node_name.trim().to_string(), node_data);
        // println!("{:?}",node_data);
        // println!("{}", node_name);
    }
    println!("{:?}",node_map);

    
    // Start at AAA and follow navigation until ZZZ is reached.
    const START_POINT : &str = "AAA";
    const END_POINT : &str  = "ZZZ";
    let mut current_point : &str = START_POINT;
    let mut nav_index : usize = 0;
    let mut num_of_steps : i32 = 0;
    while current_point != END_POINT {
        let direction : char = navigation.chars().nth(nav_index).unwrap_or('L');
        println!("curr point {}", current_point);
        let node_data = node_map.get(current_point).unwrap();
        if direction == 'L' {
            current_point = &node_data.left;
        }
        else {
            current_point = &node_data.right;
        }
        nav_index = (nav_index+1)%navigation.len();
        num_of_steps += 1;
    }
    println!("num of steps : {}", num_of_steps);



}
