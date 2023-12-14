use std::collections::HashSet;

fn vertical_symmetry(pattern:Vec<Vec<char>>, col_idx:usize) -> usize {
    let mut left: HashSet<(usize, usize)> = HashSet::new();
    let mut right: HashSet<(usize, usize)> = HashSet::new();
    let mut depth = 0;
    while left.eq(&right) && col_idx + depth < pattern[0].len() && col_idx as i32 - 1 - depth as i32 >= 0 {
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

fn smudge_patterns(pattern: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut smudged_patterns: Vec<Vec<Vec<char>>> = Vec::new();
    for col in 0..pattern.len() {
        for row in 0..pattern[0].len() {

            let mut smudged_pattern = pattern.clone();
            smudged_pattern[col][row] = if pattern[col][row] == '.' { '#' } else { '.' };
            smudged_patterns.push(smudged_pattern);

        }
    }
    smudged_patterns
}

fn print_pattern(pattern: Vec<Vec<char>>) {
    for row in pattern {
        println!("{:?}", row.into_iter().collect::<String>())
    }
    println!("");
}

fn find_perfect_line(pattern:Vec<Vec<char>>, ignore:Option<(Option<usize>, Option<usize>)>) -> (Option<usize>, Option<usize>) {
    let mut col_symmetry: Option<usize> = None;
    let mut row_symmetry: Option<usize> = None;
    let to_ignore = ignore.unwrap_or((None, None));
    for col in 1..pattern[0].len() {
        let symmetry = vertical_symmetry(pattern.clone(), col);
        if col + symmetry == pattern[0].len() || col - symmetry == 0 {
            if Some(col) != to_ignore.1 {
                col_symmetry = Some(col);
            }
        }
    }
    for row in 1..pattern.len() {
        let symmetry = horizontal_symmetry(pattern.clone(), row);
        if row + symmetry == pattern.len() || row - symmetry == 0 {
            if Some(row) != to_ignore.0 {
                row_symmetry = Some(row);
            }
        }
    }
    (row_symmetry, col_symmetry)
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
    let mut sum_b = 0;
    for (num, pattern) in patterns.iter().enumerate() {

        let lines = find_perfect_line((*pattern).clone(), None);
        match lines {
            (Some(row), None) => sum_a += row * 100,
            (None, Some(col)) => sum_a += col,
            _ => panic!("Unexpected Row AND Column")
        }
        for smudged_pattern in smudge_patterns((*pattern).clone()) {
            let perfect_line = find_perfect_line(smudged_pattern.clone(), Some(lines.clone()));
            if perfect_line != lines {
                match perfect_line {
                    (row, _) if row != lines.0 && row.is_some() => {
                        sum_b += row.unwrap() * 100;
                        break;
                    },
                    (_, col) if col != lines.1 && col.is_some() => {
                        sum_b += col.unwrap();
                        break;
                    },
                    _ => ()
                }
            }
        }
    }
    (Some(sum_a.to_string()), Some(sum_b.to_string()))
}