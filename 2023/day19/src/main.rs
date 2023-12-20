use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Condition {
    operand: char,
    operand2: char,
    operator: char,
    value: i64,
    value2: i64,
    send_to: String,
}

fn main() {
    let mut file = std::fs::File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let mut workflow_map: HashMap<String, Vec<Condition>> = HashMap::new();

    // map key to set of values that call the workflow
    let mut call_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut caller_map: HashMap<String, Vec<String>> = HashMap::new();
    // map cost to pair of functions (s1, s2) where s1 is the calling function and s2 is the callee
    let mut cost_map: HashMap<(String, String), i64> = HashMap::new();
    // set of workflows that can result in an approval
    let mut approve_set: HashSet<String> = HashSet::new();

    let mut input_vec: Vec<HashMap<char, i64>> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            break;
        }
        // Read workflow
        let line_split: Vec<&str> = line.split('{').collect();
        let workflow_name = line_split[0];
        let mut conditions: Vec<Condition> = Vec::new();

        // add to approval set if it contains an approval
        // add all called functions to call map
        // add call cost to cost map
        // Extract conditions
        let formatted = line_split[1].replace("}", "");
        let condition_strings: Vec<&str> = formatted.split(',').collect();

        let mut cost_sum: i64 = 0;
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
                    cost_sum = operator_target_split[1].parse::<i64>().unwrap();
                } else {
                    operator_target_split = cond_split[0].split('>').collect();
                    operator = '>';
                    cost_sum = 4000 - operator_target_split[1].parse::<i64>().unwrap();
                }
                send_to = cond_split[1];
                operand = operator_target_split[0].chars().nth(0).unwrap();
                value = operator_target_split[1]
                    .parse::<i64>()
                    .expect("Unable to parse value");
            }

            // Increment cost_sum
            if !call_map.contains_key(&send_to.to_string()) {
                let mut call_vec: Vec<String> = Vec::new();
                call_vec.push(workflow_name.to_string());
                call_map.insert(send_to.to_string(), call_vec);
            } else if let Some(vec_ref) = call_map.get_mut(&send_to.to_string()) {
                vec_ref.push(workflow_name.to_string());
            }

            // Add to approve set
            if send_to == "A" {
                approve_set.insert(workflow_name.to_string());
            }

            // Add to cost map
            cost_map.insert((workflow_name.to_string(), send_to.to_string()), cost_sum);

            let condition = Condition {
                operand,
                operand2: 'n',
                operator,
                value,
                value2: -1,
                send_to: send_to.to_string(),
            };
            conditions.push(condition);
        }

        workflow_map.insert(workflow_name.to_string(), conditions);
    }

    //println!("{:?}", cost_map);
    //println!("{:?}", call_map);
    //println!("{:?}", approve_set);
    // The problem is we are multiplying across branches
    // need to create multiple paths, multiply across the paths and then sum the paths together

    let mut calling_workflow_set: HashSet<String> = approve_set;

    let mut combinations: i128 = 1;
    let mut next_workflow_set: HashSet<String> = HashSet::new();
    while !calling_workflow_set.is_empty() {
        println!("WORKFLOW SET\n {:?}", calling_workflow_set);
        for workflow in calling_workflow_set.iter() {
            println!("COMBINATIONS {}", combinations);
            if let Some(approve_cost) = cost_map.get_mut(&(workflow.to_string(), "A".to_string())) {
                combinations *= (*approve_cost as i128);
            }

            for calling_workflow in call_map.get(workflow).unwrap().iter() {
                next_workflow_set.insert(calling_workflow.to_string());

                if let Some(call_cost) =
                    cost_map.get_mut(&(calling_workflow.to_string(), workflow.to_string()))
                {
                    combinations *= (*call_cost as i128);
                }
            }
        }
        calling_workflow_set = next_workflow_set.clone();
    }

    println!("Total sum {}", combinations);
}

// essentially a preorder traversal through a tree with the "in" workflow as the root and its
// children are the callee's.
// We use this to compute the combinations
fn get_combination(
    workflow_map: &HashMap<String, Vec<Condition>>,
    value_map: HashMap<char, Condition>,
    start_call: (String, String),
    combinations: i128,
) -> i128 {
    // base condition met
    // Compute combinations possible based on conditions required to get to this point
    if start_call.0 == "A" {
        combinations = 1;
        for values in value_map.values() {
            // Condition has both > and <
            if values.value2 != -1 {
                let mut less_than_value: i128 = 0;
                let mut greater_than_value: i128 = 0;
                if values.operator == '<' {
                    less_than_value = values.value as i128;
                    greater_than_value = values.value2 as i128;
                } else {
                    less_than_value = values.value2 as i128;
                    greater_than_value = values.value as i128;
                }
                combinations *= (greater_than_value - less_than_value);
            }
            // Condition only has one operator
            else if values.operator == '<' {
                combinations *= values.value as i128;
            } else {
                combinations *= (4000 - values.value as i128);
            }
        }
        // compute combinations
        return combinations;
    }

    // iterate through conditions
    let mut is_approved: bool = false;
    let mut temp_comb: i128 = combinations;
    for child in workflow_map.get(&start_call.0).iter() {
        // add branches
        // assume valid branch move along

        // Check if there is already a pre-existing condition in the value_map and update the
        // condition if necessary

        combinations += get_combination(workflow_map, value_map, (this, next), temp_comb);

        // assume invalid branch, update value_map
    }

    return 0;
}
