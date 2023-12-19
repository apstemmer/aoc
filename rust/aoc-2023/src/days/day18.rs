use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn print_grid(points: &HashSet<(i64, i64)>, dims: (i64, i64, i64,i64)) {
    let buffer:i64 = 0;
    let mut grid = vec![vec!['.'; (dims.3 - dims.2 + buffer) as usize  + 1]; (dims.1 - dims.0 + buffer) as usize  + 1];
    for x in (0..grid.len()).rev() {
        for y in 0..=grid[0].len() {
            match points.get(&(x as i64 + dims.0, y as i64 + dims.2)) {
                Some(..) => grid[x as usize][y as usize] = '#',
                None => ()
            }
        }
        println!("r {} {:?}", x as i64 + dims.0, grid[x as usize].iter().collect::<String>());
    }
}

fn shoelace(open_nodes: &Vec<(i64, i64)>) -> i64 {
    let mut nodes = open_nodes.clone();
    nodes.push(nodes[0].clone());
    let mut left_lace = 0;
    let mut right_lace = 0;
    for i in 0..nodes.len()-1 {
        left_lace += nodes[i].0 * nodes[i+1].1;
        right_lace += nodes[i].1 * nodes[i+1].0;
    }
    (left_lace - right_lace).abs() / 2
}

fn find_area_from_moves(moves: Vec<(char, i64)>) -> i64 {
    let mut nodes: Vec<(i64, i64)> = Vec::new();
    let mut perimeter: i64 = 0;
    let mut curr = (0, 0);

    for (direction, distance) in moves {
        nodes.push(curr.clone());
        match direction {
            'U' => curr = (curr.0 + distance, curr.1),
            'R' => curr = (curr.0, curr.1 + distance),
            'D' => curr = (curr.0 - distance, curr.1),
            'L' => curr = (curr.0, curr.1 - distance),
            _ => ()
        }
        perimeter += distance;

    }
    shoelace(&nodes) + perimeter / 2 + 1
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut moves: Vec<(char, i64)> = Vec::new();
    let mut nodes: Vec<(i64, i64)> = Vec::new();
    let mut hex_moves: Vec<(char, i64)> = Vec::new();

    for row in input {
        let mut split = row.split_whitespace().collect::<Vec<&str>>();
        let record = (split[0].parse::<char>().unwrap(), split[1].parse::<i64>().unwrap());
        // println!("{:?}", split);
        let hex = i64::from_str_radix(&split[2][2..split[2].len() - 1], 16).unwrap();
        let direction = match hex & 0xF {
            0 => 'R',
            1 => 'D',
            2 => 'L',
            3 => 'U',
            _ => panic!("Could not find valid direction!")
        };
        let length = hex >> 4;
        moves.push(record);
        hex_moves.push((direction, length));
    }
    let area: i64 = find_area_from_moves(moves);
    let hex_area:i64 = find_area_from_moves(hex_moves);
    (Some(area.to_string()), Some(hex_area.to_string()))
}