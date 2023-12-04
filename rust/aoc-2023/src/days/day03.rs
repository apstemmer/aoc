use std::collections::HashMap;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {

    let schematic: Vec<Vec<char>> = input.into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut is_part = false;
    let mut curr_num = 0;
    let mut gear_map: HashMap<(usize, usize), u32> = HashMap::new();
    let mut gear:Option<(usize, usize)> = None;

    let get_indices = |r:i32, c:i32| -> Vec<(i32, i32)> {
        let mut indices: Vec<(i32, i32)> = Vec::new();

        let mut add_index = |dx:i32, dy:i32| {
            if r + dx >= 0 && c + dy >= 0 &&
                schematic.len() as i32 > r + dx && schematic[0].len() as i32 > c + dy {
                indices.push((dx, dy));
            }
        };

        add_index(-1, -1);
        add_index(-1, 0);
        add_index(-1, 1);
        add_index(0, -1);
        add_index(0, 1);
        add_index(1, -1);
        add_index(1, 0);
        add_index(1, 1);

        indices
    };

    for (r, row) in schematic.iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            // If not . look around for symbols
            if item.is_numeric() {
                curr_num = curr_num * 10 + item.to_digit(10).unwrap();
                for (dr, dc) in get_indices(r as i32, c as i32) {
                    let coord = (((r as i32) + dr) as usize, ((c as i32) + dc) as usize);
                    let mut aux: char = schematic[coord.0][coord.1];
                    if aux == '*' && gear != Some((coord.0, coord.1)){
                        if gear != None {
                            println!("{}, {}", r, c);
                            panic!("Should Never Happen!!")
                        }
                        gear = Some((coord.0, coord.1));
                    }
                    if !aux.is_numeric() && aux != '.' {
                        // We've found a symbol
                        is_part = true;
                    }
                }
            } else {
                // If at end of num (curr_num > 0) add tot total if a part
                if is_part {
                    sum_a += curr_num;
                }
                match gear {
                    Some(gear_coord) if !gear_map.contains_key(&gear_coord) => {
                        gear_map.insert(gear_coord, curr_num);
                    },
                    Some(gear_coord) => {
                        sum_b += gear_map.get(&gear_coord).unwrap() * curr_num;
                    },
                    None => ()
                }
                curr_num = 0;
                is_part = false;
                gear = None;
            }
        }
    }
    (Some(sum_a.to_string()), Some(sum_b.to_string()))
}