use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Condition {
    operand: char,
    operator: char,
    value: i64,
    send_to: String,
}

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let mut workflow_map: HashMap<String, Vec<Condition>> = HashMap::new();
    let mut input_vec: Vec<HashMap<char, i64>> = Vec::new();
    let mut reading_workflow: bool = true;
    for line in contents.lines() {
        if line.is_empty() {
            reading_workflow = false;
            continue;
        }

        if reading_workflow {
            read_workflow(&mut workflow_map, line);
        } else {
            read_input(&mut input_vec, line);
        }
    }

    let mut accepted_sum: i64 = 0;
    // Process input
    for input in input_vec.iter() {
        //println!("PROCESING INPUT : {:?}", input);
        // Start at the IN VEC
        let mut workflow: &Vec<Condition> = workflow_map.get("in").unwrap();
        let mut is_accepted: bool = true;
        let mut in_workflow: bool = true;
        while in_workflow {
            // work through the workflow
            for condition in workflow {
                //println!("CONDITION {:?}", condition);
                // Direct send to condition
                if condition.operator == '!' {
                    if condition.send_to == "A" {
                        in_workflow = false;
                        break;
                    }
                    if condition.send_to == "R" {
                        is_accepted = false;
                        in_workflow = false;
                        break;
                    }
                    //println!("SEND TO {}", condition.send_to);
                    workflow = workflow_map.get(&condition.send_to).unwrap();
                    break;
                }

                // Proces condition
                let value = input.get(&condition.operand).unwrap();
                let target_value = condition.value;
                if condition.operator == '<' && *value < target_value
                    || condition.operator == '>' && *value > target_value
                {
                    if condition.send_to == "A" {
                        in_workflow = false;
                        break;
                    }
                    if condition.send_to == "R" {
                        is_accepted = false;
                        in_workflow = false;
                        break;
                    }
                    // Jump to send_to
                    //println!("SEND TO {}", condition.send_to);
                    workflow = workflow_map.get(&condition.send_to).unwrap();
                    break;
                }
            }
        }

        //println!("ACCEPTED : {}", is_accepted);
        if is_accepted {
            for value in input.values() {
                accepted_sum += value;
            }
        }
    }

    println!("sum : {}", accepted_sum);
}

fn read_input(input_vec: &mut Vec<HashMap<char, i64>>, line: &str) {
    let formatted_line = line.replace(&['{', '}'][..], "");
    let inputs: Vec<&str> = formatted_line.split(',').collect();

    let mut map: HashMap<char, i64> = HashMap::new();
    for input in inputs {
        let name_value_split: Vec<&str> = input.split('=').collect();
        let name = name_value_split[0].chars().nth(0).unwrap();
        let value = name_value_split[1].parse::<i64>().unwrap();
        map.insert(name, value);
    }

    input_vec.push(map);
}

fn read_workflow(workflow_map: &mut HashMap<String, Vec<Condition>>, line: &str) {
    // Read workflow
    let line_split: Vec<&str> = line.split('{').collect();
    let workflow_name = line_split[0];
    let mut conditions: Vec<Condition> = Vec::new();

    // Extract conditions
    let formatted = line_split[1].replace("}", "");
    let condition_strings: Vec<&str> = formatted.split(',').collect();

    for cond in condition_strings.iter() {
        let mut operand: char = 'a';
        let mut operator: char = 'a';
        let mut value: i64 = 0;
        let mut send_to: &str = "workspace_name";

        // single value send
        if !cond.contains(':') {
            // Single send operator, informs the code that the vale contains no condition
            operator = '!';
            send_to = cond;
        }
        // standard condition
        else {
            let cond_split: Vec<&str> = cond.split(':').collect();
            let mut operator_target_split: Vec<&str> = Vec::new();
            if cond_split[0].contains('<') {
                operator_target_split = cond_split[0].split('<').collect();
                operator = '<';
            } else {
                operator_target_split = cond_split[0].split('>').collect();
                operator = '>';
            }
            send_to = cond_split[1];
            operand = operator_target_split[0].chars().nth(0).unwrap();
            value = operator_target_split[1]
                .parse::<i64>()
                .expect("Unable to parse value");
        }

        let condition = Condition {
            operand,
            operator,
            value,
            send_to: send_to.to_string(),
        };
        conditions.push(condition);
    }

    workflow_map.insert(workflow_name.to_string(), conditions);
}
