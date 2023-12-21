use std::collections::HashMap;
use std::string::String;

#[derive(Clone, Debug)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    pub fn new(min:i32, max:i32) -> Range{
        Range { min, max }
    }
}

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

fn range_overlap(range_a:&Range, range_b:&Range) -> Option<Range> {
    let start = std::cmp::max(range_a.min, range_b.min);
    let end = std::cmp::min(range_a.max, range_b.max);

    if start < end {
        Some(Range::new(start, end))
    } else {
        None // No overlap
    }
}

fn range_split(sub_range: &Range, super_range: &Range) -> Range{
    if sub_range.min == super_range.min {
        return Range::new(sub_range.max, super_range.max);
    }
    Range::new(super_range.min, sub_range.min)
}

fn process_range(range:(String, (Range, Range, Range, Range)), workflows:&HashMap<String, Vec<(i32, char, i32, String)>>) -> Vec<(String, (Range, Range, Range, Range))> {
    let mut next_ranges = Vec::new();
    let mut curr_range = range.1.clone();
    let workflow = workflows[&range.0].clone();
    for rule in workflow {
        let mut curr = curr_range.clone();
        println!("Rule: {:?} -> {:?} -> {:?}", rule, curr, next_ranges);
        match rule {
            (0, cmp, value, next ) => {
                let cmp_range = if cmp == '>' { Range::new(value, 4000) } else { Range::new(0, value) };
                let overlap = range_overlap(&curr.0, &cmp_range);
                if overlap.is_some() {
                    curr_range.0 = range_split(&overlap.clone().unwrap(), &curr.0);
                    next_ranges.push((next, (overlap.unwrap(), curr.1, curr.2, curr.3)));
                }
            }
            (1, cmp, value, next ) => {
                let cmp_range = if cmp == '>' { Range::new(value, 4000) } else { Range::new(0, value) };
                let overlap = range_overlap(&curr.1, &cmp_range);
                if overlap.is_some() {
                    curr_range.1 = range_split(&overlap.clone().unwrap(), &curr.1);
                    next_ranges.push((next, (curr.0, overlap.unwrap(), curr.2, curr.3)));
                }
            }
            (2, cmp, value, next ) => {
                let cmp_range = if cmp == '>' { Range::new(value, 4000) } else { Range::new(0, value) };
                let overlap = range_overlap(&curr.2, &cmp_range);
                if overlap.is_some() {
                    curr_range.2 = range_split(&overlap.clone().unwrap(), &curr.2);
                    next_ranges.push((next, (curr.0, curr.1, overlap.unwrap(), curr.3)));
                }
            }
            (3, cmp, value, next ) => {
                let cmp_range = if cmp == '>' { Range::new(value, 4000) } else { Range::new(0, value) };
                let overlap = range_overlap(&curr.3, &cmp_range);
                if overlap.is_some() {
                    curr_range.3 = range_split(&overlap.clone().unwrap(), &curr.3);
                    next_ranges.push((next, (curr.0, curr.1, curr.2, overlap.unwrap())));
                }
            }
            _ => panic!("No matching rule!"),
        }
    }
    println!("----------");
    next_ranges
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

    let mut ranges = Vec::new();
    let mut accepted_ranges = Vec::new();
    ranges.push((String::from("in"), (Range::new(0, 4000),
                                              Range::new(0, 4000),
                                              Range::new(0, 4000),
                                              Range::new(0, 4000))));
    while !ranges.is_empty() {
        let processed_ranges: Vec<(String, (Range, Range, Range, Range))> = process_range(ranges.pop().unwrap(), &workflows);
        for range in processed_ranges {
            match range {
                (next, r) if next == String::from("A") => accepted_ranges.push(r),
                (next, _) if next == String::from("R") => (),
                (next, r) => ranges.push((next, r)),
            }
        }
    }
    println!("{:?} -> {}", accepted_ranges, accepted_ranges.len());
    let mut sum_b: i64 = 0;
    for range in accepted_ranges {
        sum_b += (range.0.max - range.0.min) as i64 * (range.1.max - range.1.min) as i64 * (range.2.max - range.2.min) as i64 * (range.3.max - range.3.min) as i64;
    }

    (Some(sum_a.to_string()), Some(sum_b.to_string()))
}