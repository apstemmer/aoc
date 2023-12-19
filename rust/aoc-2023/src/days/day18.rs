use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn print_grid(points: &HashSet<(i32, i32)>, dims: (i32, i32, i32,i32)) {
    let buffer:i32 = 0;
    let mut grid = vec![vec!['.'; (dims.3 - dims.2 + buffer) as usize  + 1]; (dims.1 - dims.0 + buffer) as usize  + 1];
    for x in (0..grid.len()).rev() {
        for y in 0..=grid[0].len() {
            match points.get(&(x as i32 + dims.0, y as i32 + dims.2)) {
                Some(..) => grid[x as usize][y as usize] = '#',
                None => ()
            }
        }
        println!("r {} {:?}", x as i32 + dims.0, grid[x as usize].iter().collect::<String>());
    }
}

fn shoelace(open_nodes: &Vec<(i32, i32)>) -> i32 {
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
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut colored:HashSet<(i32, i32)> = HashSet::new();
    let mut moves: Vec<(char, i32)> = Vec::new();
    let mut nodes: Vec<(i32, i32)> = Vec::new();

    for row in input {
        let mut split = row.split_whitespace().collect::<Vec<&str>>();
        let record = (split[0].parse::<char>().unwrap(), split[1].parse::<i32>().unwrap());
        moves.push(record);
    }

    let mut curr = (0, 0);
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (direction, distance) in moves {
        nodes.push(curr.clone());
        for i in 1..distance + 1 {
            match direction {
                'U' => curr = (curr.0 + 1, curr.1),
                'R' => curr = (curr.0, curr.1 + 1),
                'D' => curr = (curr.0 - 1, curr.1),
                'L' => curr = (curr.0, curr.1 - 1),
                _ => ()
            }
            colored.insert(curr.clone());
            min_x = min(min_x, curr.0);
            max_x = max(max_x, curr.0);
            min_y = min(min_y, curr.1);
            max_y = max(max_y, curr.1);
        }
    }
    let area = shoelace(&nodes) + colored.len() as i32 / 2 + 1;

    println!("Nodes: {:?} -> {}", nodes, area);
    (Some(area.to_string()), None)
}