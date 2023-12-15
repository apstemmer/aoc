use std::cmp::max;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
fn hash_grid(data: &Vec<Vec<char>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for vec in data {
        vec.hash(&mut hasher);
    }
    hasher.finish()
}
fn cycle(mut grid : Vec<Vec<char>>) -> Vec<Vec<char>> {
    // North
    for col in 0..grid[0].len() {
        let mut cube = 0;
        let mut line = vec!['.'; grid.len()];
        for row in 0..grid.len() {
            match grid[row][col] {
                '#' => {
                    line[row] = '#';
                    cube = row + 1;
                },
                'O' => {
                    line[cube] = 'O';
                    cube += 1;
                }
                _ => ()
            }
        }
        for row in 0..grid.len() {
            grid[row][col] = line[row];
        }
    }
    // print_grid(grid.clone());
    // West
    for row in 0..grid.len() {
        let mut cube = 0;
        let mut line = vec!['.'; grid[0].len()];
        for col in 0..grid[0].len() {
            match grid[row][col] {
                '#' => {
                    line[col] = '#';
                    cube = col + 1;
                },
                'O' => {
                    line[cube] = 'O';
                    cube += 1;
                }
                _ => ()
            }
        }
        for col in 0..grid[0].len() {
            grid[row][col] = line[col];
        }
    }
    // print_grid(grid.clone());
    // South
    for col in 0..grid[0].len() {
        let mut cube = grid.len() - 1;
        let mut line = vec!['.'; grid.len()];
        for row in (0..grid.len()).rev() {
            match grid[row][col] {
                '#' => {
                    line[row] = '#';
                    cube = max((row as i32 - 1), 0) as usize;
                },
                'O' => {
                    line[cube] = 'O';
                    cube = max((cube as i32 - 1), 0) as usize;
                }
                _ => ()
            }
        }
        for row in 0..grid.len() {
            grid[row][col] = line[row];
        }
    }
    // print_grid(grid.clone());
    // East
    for row in 0..grid.len() {
        let mut cube = grid[0].len() - 1;
        let mut line = vec!['.'; grid[0].len()];
        for col in (0..grid[0].len()).rev() {
            match grid[row][col] {
                '#' => {
                    line[col] = '#';
                    cube = max((col as i32 - 1), 0) as usize;
                },
                'O' => {
                    line[cube] = 'O';
                    cube = max((cube as i32 - 1), 0) as usize;
                }
                _ => ()
            }
        }
        for col in 0..grid[0].len() {
            grid[row][col] = line[col];
        }
    }
    // print_grid(grid.clone());
    grid
}
fn print_grid(grid:Vec<Vec<char>>) {
    for row in grid {
        println!("{:?}", row.into_iter().collect::<String>())
    }
    println!("");
}

fn find_load(grid:Vec<Vec<char>>) -> usize {
    let mut load = 0;
    let max_weight = grid.len();
    for col in 0..grid[0].len() {
        let mut weight = max_weight;
        for row in 0..grid.len() {
            match grid[row][col] {
                '#' => weight = max_weight - (row + 1),
                'O' => {
                    load += weight;
                    weight -= 1;
                }
                _ => ()
            }
        }
    }
    load
}

fn find_east_load(grid:Vec<Vec<char>>) -> usize {
    let mut load = 0;
    let max_weight = grid.len();
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            match grid[row][col] {
                'O' => {
                    load += max_weight - row;
                }
                _ => ()
            }
        }
    }
    load
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut grid: Vec<Vec<char>> = input.iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();
    let sum_a = find_load(grid.clone());
    let sum_b = 0;
    let mut hashes: HashMap<u64, i32> = HashMap::new();
    for i in  0..2000 {
        if i % 10000 == 0 {
            println!("{} cycles", i);
        }
        grid = cycle(grid.clone());
        let hashed: u64 = hash_grid(&grid);
        match hashes.get(&hashed.clone()) {
            Some(found) => {
                println!("Found a Duplicate! {} -> {} -> {}", found, i, find_east_load(grid.clone()));
            },
            None => {hashes.insert(hashed,i);},
        }
    }

    (Some(sum_a.to_string()), None)
}