use std::collections::HashSet;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut grid : Vec<Vec<char>> = Vec::new();
    for row in input {
        grid.push( row.chars().collect());
    }
    let mut start = (0, 0);
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'S' {
                start = (r as i32, c as i32);
            }
        }
    }
    println!("Start: {:?}", start );

    let mut plots: HashSet<(i32, i32)> = HashSet::new();
    plots.insert(start);
    grid[start.0 as usize][start.1 as usize] = '.';

    for step in 1..=64 {
        let mut next_plots = HashSet::new();
        for (row, col) in plots {
            let r: i32 = row as i32;
            let c: i32 = col as i32;
            // Up
            if r - 1 >= 0 && grid[r as usize - 1][c as usize] != '#' {
                next_plots.insert((r-1, c));
            }
            // Down
            if r + 1 < grid.len() as i32 && grid[r as usize + 1][c as usize] != '#' {
                next_plots.insert((r+1, c));
            }
            // Left
            if c - 1 >= 0 && grid[r as usize][c as usize - 1] != '#' {
                next_plots.insert((r, c-1));
            }
            // Right
            if c + 1 < grid[0].len() as i32 && grid[r as usize][c as usize + 1] != '#' {
                next_plots.insert((r, c+1));
            }
        }
        println!("Next ({}): {:?}", step, next_plots.len());
        plots = next_plots.clone();
    }
    println!("Final: {}", plots.len());
    (Some(plots.len().to_string()), None)
}