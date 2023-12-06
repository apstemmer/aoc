use std::cmp::min;
use std::collections::HashMap;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut maps : Vec<HashMap<(i64, i64), i64>> = Vec::new();
    // Build List of mappings
    let seeds:Vec<i64> = input[0].split_whitespace()
        .filter(|s| !s.contains("seeds:"))
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    let seeds_b = seeds.clone();

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

    let mut location_a: i64 = i64::MAX;
    let mut location_b: Option<i64> = None;

    for seed in seeds {
        let mut source = seed;
        let mut dest = None;
        for map in maps.iter() {
            for ((start, end), offset) in map {
                if *start < source && source < *end {
                    dest = Some(source - *start + offset);
                }
            }
            if dest.is_none() {
                dest = Some(source);
            }
            // println!("{} -> {}", source, dest.unwrap());
            source = dest.unwrap();
            dest = None;
        }
        location_a = min(location_a, source);
    }

    for trial in 0_i64..100_000_000 {
        if trial % 10_000 == 0 {
            println!("Trialling: {}", trial);
        }
        let mut src = trial;
        let mut dst = None;
        for map in maps.iter().rev() {
            for ((start, end), offset) in map {
                let range = *end - *start;
                if offset + range > src && src >= *offset {
                    dst = Some(src - *offset + start)
                }
            }
            if dst.is_none() {
                dst = Some(src);
            }
            src = dst.unwrap();
            dst = None;
        }
        for pair in seeds_b.chunks(2) {
            // println!("Considering if {}, {:?}, {:?}", trial, src, pair);
            if pair[0] <= src && src < pair[0] + pair[1] {
                location_b = Some(trial)
            }
        }
        if location_b != None {
            break;
        }
    }
    (Some(location_a.to_string()), Some(location_b.unwrap().to_string()))
}

// Total Search space: 1,246,535,471