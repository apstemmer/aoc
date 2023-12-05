use std::collections::HashMap;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut maps : Vec<HashMap<(i64, i64), i64>> = Vec::new();
    // Build List of mappings
    let seeds:Vec<i64> = input[0].split_whitespace()
        .filter(|s| !s.contains("seeds:"))
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    let mut map_idx = 0;
    maps.push(HashMap::new());
    for line in input.iter().skip(3) {
        if line.contains("map:") {
            map_idx += 1;
            maps.push(HashMap::new());
            continue;
        }
        else if line.is_empty() {
            continue;
        }
        let mut nums: Vec<i64> = line.split_whitespace()
            .map(|pos| pos.parse::<i64>().unwrap())
            .collect();

        maps.get_mut(map_idx).unwrap()
            .insert((nums[1], nums[1]+nums[2]), nums[0]);
        println!("{:?}", maps[map_idx]);
    }

    let mut locations = Vec::new();
    for seed in seeds {
        println!("New Seed: {}", seed);
        let mut source = seed;
        let mut dest = None;
        for map in maps.clone().into_iter() {
            for ((start, end), offset) in &map {
                if *start < source && source < *end {
                    dest = Some(source - *start + offset);
                }
            }
            if dest.is_none() {
                dest = Some(source);
            }
            println!("{} -> {}", source, dest.unwrap());
            source = dest.unwrap();
            dest = None;
        }
        locations.push(source)
    }
    println!("{:?}", locations);
    (Some(locations.iter().min().unwrap().to_string()), None)
}