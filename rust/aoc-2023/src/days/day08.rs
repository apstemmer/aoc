use std::collections::{HashMap, HashSet};

fn find_steps_to_end(instructions: Vec<char>, nodes:HashMap<String, (String, String)>, start: String) -> i64{
    let mut steps: i64 = 0;
    let mut curr_node = start.clone();
    while !curr_node.ends_with('Z') {
        if steps % 1000 == 0{
            println!("Step {}: {}", start, steps);
        }
        let index = (steps % instructions.len() as i64) as usize;
            let adjacent = nodes.get(curr_node.as_str()).unwrap().clone();
            match instructions[index] {
                'L' => curr_node = adjacent.0,
                'R' => curr_node = adjacent.1,
                _ => panic!("Invalid L/R Character!")
            }
        steps += 1;
    }
    steps
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let instructions: Vec<char> = input[0].chars().collect();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let mut ghosts: Vec<String> = Vec::new();
    let mut ends: HashSet<String> = HashSet::new();

    for node in input.iter().skip(2) {
        println!("{}", node);
        let parts: Vec<&str> = node.split('=').collect();

        let second = parts[1].trim();
        let second_trimmed = &second[1..second.len()-1]; // Removes '(' and ')'
        let values: Vec<&str> = second_trimmed.split(',').map(str::trim).collect();

        let node_name = parts[0].trim();
        let left = values[0].clone();
        let right = values[1].clone();

        println!("{} -> {} {}", node_name, left, right);
        nodes.insert(node_name.to_string(), (left.to_string(), right.to_string()));
        if node_name.ends_with('A') {
            ghosts.push(node_name.to_string());
        }
        if node_name.ends_with('Z') {
            ends.insert(node_name.to_string());
        }
    }

    println!("Start: {:?}, Ends: {:?}", ghosts, ends);

    let mut steps_a: i64 = 0;
    let mut steps_b: i64 = 0;
    let mut curr_node: String = String::from("AAA");
    while curr_node != "ZZZ" {
        if steps_a % 1000 == 0{
            println!("Step: {}", steps_a);
        }
        let index = (steps_a % instructions.len() as i64) as usize;
        let adjacent = nodes.get(curr_node.as_str()).unwrap().clone();
        match instructions[index] {
            'L' => curr_node = adjacent.0,
            'R' => curr_node = adjacent.1,
            _ => panic!("Invalid L/R Character!")
        }
        steps_a += 1;
    }
    let mut steps_to_end = Vec::new();
    for ghost in ghosts {
        steps_to_end.push(find_steps_to_end(instructions.clone(), nodes.clone(), ghost.clone()));
    }
    println!("Steps: {:?}", steps_to_end);

    (Some(steps_a.to_string()), Some(steps_b.to_string()))
}