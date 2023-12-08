use std::collections::HashMap;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let instructions: Vec<char> = input[0].chars().collect();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

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
    }

    let mut steps: i64 = 0;
    let mut curr_node: String = String::from("AAA");
    while curr_node != "ZZZ" {
        if steps % 1000 == 0{
            println!("Step: {}", steps);
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

    (Some(steps.to_string()), None)
}