use std::collections::HashMap;

pub fn HASH(sequence: String) -> i64 {
    let ascii: Vec<i64> = sequence.chars().map(|c| c as i64).collect();
    let mut hash:i64 = 0;
    for val in ascii.iter() {
        hash += val;
        hash = hash * 17;
        hash = hash % 256;
    }
    println!("{} -> {}", sequence, hash);
    hash
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let sequences: Vec<Vec<i64>> = input
        .iter()
        .flat_map(|s| s.split(',')
            .map(|substring| substring.chars().map(|c| c as i64).collect())
        )
        .collect();
    let mut sum_a:i64 = 0;
    for sequence in sequences {
        let mut hash:i64 = 0;
        for val in sequence.iter() {
            hash += val;
            hash = hash * 17;
            hash = hash % 256;
        }
        sum_a += hash
    }
    let steps: Vec<_> = input[0].split(',').collect();
    let mut map: HashMap<i64, Vec<(String, i64)>> = HashMap::new();
    for step in steps {
        if step.contains('=') {
            let mut split_step = step.split('=');
            let label = split_step.next().unwrap().to_string();
            let lens = split_step.next().unwrap().to_string().parse::<i64>().unwrap();
            match map.get_mut(&HASH(label.clone())) {
                Some(lenses) => {
                    let mut found = false;
                    for i in 0..lenses.len() {
                        if lenses[i].0 == label {
                            lenses[i] = (label.clone(), lens.clone());
                            found = true;
                        }
                    }
                    if !found {
                        lenses.push((label.clone(), lens.clone()));
                    }
                },
                None => {
                    map.insert(HASH(label.clone()), vec![(label.clone(), lens.clone())]);
                }
            }
            println!("{} -> {} {}", step, label, lens);

        } else if step.contains('-') {
            let mut split_step = step.split('-');
            let label = split_step.next().unwrap().to_string();
            match map.get_mut(&HASH(label.clone())) {
                Some(lenses) => {
                    // If the labels match, remove it
                    if let Some(index) = lenses.iter().position(|lens| lens.0 == label) {
                        lenses.remove(index);
                    }
                },
                None => ()
            }
            println!("{} -> {}", step, label);
        }
        println!("MAP: {:?}", map);
    }
    let mut focusing_power = 0;
    for key in map.keys() {
        let mut lenses: Vec<(String, i64)> = map[key].clone();
        for i in 0..lenses.len() {
            let curr_power = (*key + 1) * (i as i64 + 1) * lenses[i].1;
            focusing_power += curr_power;
            println!("({}, {}) -> {}", lenses[i].0, lenses[i].1, curr_power);
        }
    }
    (Some(sum_a.to_string()), Some(focusing_power.to_string()))
}