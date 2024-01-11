use std::collections::HashMap;
use std::io::Read;

pub mod modules;
use crate::modules::*;


fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let mut module_map: HashMap<String, Box<dyn OutSignal>> = HashMap::new();

    // Pass 1 create modules
    for line in contents.lines() {
        //println!("{:?}", line);
        let split_line: Vec<&str> = line.split("->").collect();
        let type_and_id = split_line[0];

        let module_type = type_and_id.chars().nth(0).unwrap();
        let module_id = type_and_id.get(1..).unwrap().trim();

        let output_list: Vec<&str> = split_line[1].split(',').collect();
        let outputs: Vec<String> = output_list.iter().map(|&s| s.trim().to_string()).collect();

        match module_type {
            '%' => {
                let new_flip_flop = Box::new(FlipFlopModule {
                    state: false,
                    output_pins: outputs,
                });

                module_map.insert(module_id.to_string(), new_flip_flop);
            }
            '&' => {
                let new_conjunction = Box::new(ConjunctionModule {
                    input_pins: HashMap::new(),
                    output_pins: outputs,
                });

                module_map.insert(module_id.to_string(), new_conjunction);
            }
            'b' => {
                let new_broadcast = Box::new(BroadcastModule {
                    output_pins: outputs,
                });

                module_map.insert("broadcaster".to_string(), new_broadcast);
            }
            _ => {}
        }
    }

    // Pass 2 link input pins to conjunction modules
    for line in contents.lines() {
        //println!("{:?}", line);
        let split_line: Vec<&str> = line.split("->").collect();
        let type_and_id = split_line[0];

        let module_id = type_and_id.get(1..).unwrap().trim();
        let output_list: Vec<&str> = split_line[1].split(',').collect();
        let outputs: Vec<String> = output_list.iter().map(|&s| s.trim().to_string()).collect();

        for output in outputs {
            //println!("OUTPUT : {}", output);
            match module_map.get(&output) {
                Some(value) => {
                    if value.get_type() != EModuleType::Conjunction {
                        continue;
                    }
                }
                None => {
                    //println!("No value found");
                    continue;
                }
            }

            //println!("ADDING {} to {}", module_id, output);
            if let Some(conjunction) = module_map.get_mut(&output) {
                conjunction.add_input_pin(module_id);                                                                        
            }
        }
    }

    // Main loop
    let mut low_count = 0;
    let mut high_count = 0;

    for n in  1..=1000 {
        let  (itr_low_count, itr_high_count) = press_button(&mut module_map);
        low_count += itr_low_count;
        high_count += itr_high_count;
    }

    println!("HC : {} LC : {}", high_count, low_count);
    println!("Result : {}", high_count*low_count);
}

fn press_button (module_map: &mut HashMap<String,Box<dyn OutSignal>>) -> (i64, i64) {
    let mut queue: Vec<(String, String, bool)> = Vec::new();
    // Main loop
    let mut low_count: i64 = 1;
    let mut high_count: i64 = 0;
    queue.push(("button".to_string(), "broadcaster".to_string(), false));

    // Need to account for the flip flops states (On or Off)
    while !queue.is_empty() {
        // Retrieve queued module
        let next_packet: (String, String, bool) = queue.pop().unwrap();

        //print_packet(&next_packet);
        if !module_map.contains_key(&next_packet.1) {
            continue;
        }
        let module: &mut Box<dyn OutSignal> = module_map.get_mut(&next_packet.1).unwrap();

        if module.get_type() == EModuleType::FlipFlop && next_packet.2 {
            continue;
        }

        // Process incoming signal using module
        let out: bool = module.out_signal(&next_packet.0, next_packet.2);
        let output_pin_list: &Vec<String> = module.get_output_list();

        // send to output pins
        for output_pin in output_pin_list {
            // ( Calling Module, Receiving Module, Out Signal)
            queue.insert(0, (next_packet.1.to_string(), output_pin.to_string(), out));
            if out == true {
                high_count += 1;
            } else {
                low_count += 1;
            }
        }
    }

    return (low_count, high_count);
}

fn print_packet(tuple: &(String, String, bool)) {
    let mut signal = "low";
    if tuple.2 {
        signal = "high";
    }
    println!(" {} -{}-> {}", tuple.0, signal, tuple.1);
}
