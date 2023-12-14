use std::collections::HashSet;

fn vertical_symmetry(pattern:Vec<Vec<char>>, col_idx:usize) -> usize {
    let mut left: HashSet<(usize, usize)> = HashSet::new();
    let mut right: HashSet<(usize, usize)> = HashSet::new();
    let mut depth = 0;
    while left.eq(&right) && col_idx + depth < pattern[0].len() && col_idx as i32 - 1 - depth as i32 >= 0 {
        println!("Considering col: {} at depth: {}", col_idx, depth);
        for (r, row) in pattern.iter().enumerate() {
            match row[col_idx - depth - 1] {
                '#' => {
                    left.insert((depth, r));
                },
                _ => ()
            }
            match row[col_idx + depth] {
                '#' => {
                    right.insert((depth, r));
                },
                _ => ()
            }
        }
        depth += 1;
    }
    if left.eq(&right) {
        depth += 1;
    }
    println!("{:?} | {:?} -> {} {} {}", left, right, left.eq(&right), pattern[0].len(), col_idx as i32 - 1 - depth as i32 >= 0);
    depth - 1
}

fn horizontal_symmetry(pattern:Vec<Vec<char>>, row_idx:usize) -> usize {
    let mut top: Vec<char> = Vec::new();
    let mut bot: Vec<char> = Vec::new();
    let mut depth = 0;
    while top.eq(&bot) && row_idx + depth < pattern.len() && row_idx as i32 - 1 - depth as i32 >= 0 {
        top = pattern[row_idx - depth - 1].clone();
        bot = pattern[row_idx + depth].clone();
        depth += 1;
    }
    if top.eq(&bot) {
        depth += 1;
    }
    depth - 1
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();
    patterns.push(Vec::new());
    for row in input {
        if row.is_empty() {
            patterns.push(Vec::new());
        } else {
            let index = patterns.len() - 1;
            let chars:Vec<char> = row.chars().collect();
            if let Some(inner_vec) = patterns.get_mut(index) {
                inner_vec.push(chars.clone());
            }
        }
    }
    let mut sum_a = 0;
    for pattern in &patterns {
        println!("New Pattern!");
        let mut col_symmetry: Option<usize> = None;
        let mut row_symmetry: Option<usize> = None;
        for col in 1..pattern[0].len() {
            let symmetry = vertical_symmetry(pattern.clone(), col);
            if col + symmetry == pattern[0].len() || col - symmetry == 0 {
                // There is perfect symmetry
                println!("Perfect Col! {} has {} symmetry", col, symmetry);
                if let Some(col_seen) = col_symmetry {
                    println!("Duplicate! {:?}", pattern);
                }
                col_symmetry = Some(col);
                // break;
            }
            println!("Column {} has {}", col, symmetry);
        }
        if let Some(col) = col_symmetry {
            for row in pattern {
                println!("{:?}", row);
            }
            sum_a += col;
            continue;
        }
        for row in 1..pattern.len() {
            let symmetry = horizontal_symmetry(pattern.clone(), row);
            if row + symmetry == pattern.len() || row - symmetry == 0 {
                // There is perfect symmetry
                println!("Perfect Row! {} has {} symmetry", row, symmetry);
                if let Some(col_seen) = col_symmetry {
                    println!("Duplicate Row1! {}, New {}", col_seen, symmetry);
                }
                if let Some(row_seen) = row_symmetry {
                    println!("Duplicate Row! {}, New {}", row_seen, row);
                }
                row_symmetry = Some(row);
                // break;
            }
            println!("Row {} has {}", row, symmetry);
        }
        if let Some(row) = row_symmetry {
            sum_a += row * 100;
            continue;
        }
    }
    println!("{:?}", patterns);
    (Some(sum_a.to_string()), None)
}