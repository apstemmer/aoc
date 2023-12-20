use std::collections::HashMap;

fn is_accepted(part:(i32, i32, i32, i32), workflows: &HashMap<String, Vec<(i32, char, i32, String)>>) -> bool{
    let mut workflow_name = String::from("in");
    while workflow_name != String::from("R") && workflow_name != String::from("A") {
        let workflow: &Vec<(i32, char, i32, String)> = workflows.get(&workflow_name).unwrap();
        for rule in workflow {
            let upcoming = match rule {
                (0, '>', value, next ) if part.0 > *value => next.clone(),
                (0, '<', value, next ) if part.0 < *value => next.clone(),
                (1, '>', value, next ) if part.1 > *value => next.clone(),
                (1, '<', value, next ) if part.1 < *value => next.clone(),
                (2, '>', value, next ) if part.2 > *value => next.clone(),
                (2, '<', value, next ) if part.2 < *value => next.clone(),
                (3, '>', value, next ) if part.3 > *value => next.clone(),
                (3, '<', value, next ) if part.3 < *value => next.clone(),
                _ => workflow_name.clone(),
            };
            if upcoming != workflow_name {
                workflow_name = upcoming;
                break;
            }
        }
    }
    match workflow_name.as_str() {
        "A" => true,
        "R" => false,
        _ => false
    }
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut workflows:HashMap<String, Vec<(i32, char, i32, String)>> = HashMap::new();
    let mut parts: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut row = 0;
    while !input[row].is_empty() {
        let splits: Vec<&str> = input[row].split(|s| "{,}".contains(s)).collect();
        let mut rules: Vec<(i32, char, i32, String)> = Vec::new();
        for rule in 1..splits.len() - 2 {
            println!("Rules {:?}, CurrRule {:?} -> {:?}", rules, splits[rule], splits);
            let rule_split: Vec<&str> = splits[rule].split(|s| "<:>".contains(s)).collect();
            rules.push((match rule_split[0] {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => 4
            }, char::from(splits[rule].as_bytes()[1]), rule_split[1].parse().unwrap(), rule_split[2].parse().unwrap()));
        }
        let default_rule = splits[splits.len()-2];
        rules.push((0, '>', 0, default_rule.to_string()));
        workflows.insert(splits[0].to_string(), rules.clone());
        row += 1;
    }
    for r in row+1..input.len() {
        let splits: Vec<&str> = input[r].split(|s| "{=,}".contains(s)).collect();
        parts.push((
            splits[2].parse::<i32>().unwrap(),
            splits[4].parse::<i32>().unwrap(),
            splits[6].parse::<i32>().unwrap(),
            splits[8].parse::<i32>().unwrap()));
        println!("{:?}", splits);
    }
    println!("{:?}", workflows);
    println!("{:?}", parts);
    let mut sum_a = 0;
    for part in parts {
        match is_accepted(part, &workflows) {
            true => sum_a += part.0 + part.1 + part.2 + part.3,
            false => (),
        }
    }
    (Some(sum_a.to_string()), None)
}