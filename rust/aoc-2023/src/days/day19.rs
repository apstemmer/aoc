use std::collections::HashMap;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut workflows:HashMap<String, Vec<(i32, char, i32, String)>> = HashMap::new();
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
    println!("{:?}", workflows);
    (None, None)
}