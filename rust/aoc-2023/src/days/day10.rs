use std::cmp::max;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    ;
    let mut start = (0i32, 0, 0);

    for (row_num, row) in input.iter().enumerate() {
        map.push( row.chars().collect());
        match row.chars().position(|c| c == 'S') {
            Some(index) => {
                map[row_num][index] = 'F';
                start = (0, row_num, index)
            },
            None => ()
        }
    }
    let mut seen: Vec<Vec<i32>> = vec![vec![-1; map[0].len()]; map.len()];
    seen[start.1][start.2] = 0;

    println!("{:?} -> {:?}", start, map[start.1][start.2]);
    let mut stack: Vec<(i32, usize, usize)> = vec![start];

    fn push_and_sort(stack: &mut Vec<(i32, usize, usize)>, item: (i32, usize, usize)) {
        stack.push(item.clone());
        stack.sort();
        stack.reverse();
    }

    let lies_in_grid = |r:i32, c:i32| -> bool { !
        0 <= r && r < map.clone().len() as i32 && 0 <= c && c <= map.clone()[0].len() as i32
    };
    let mut highest = 0;

    while stack.len() > 0 {
        let (v, r, c) = stack.pop().unwrap();
        highest = max(highest, v);
        println!("Considering: {:?}", map[r][c]);
        match map[r][c] {
            '|' => {
                if lies_in_grid(r as i32 - 1, c as i32) && seen[r-1][c] == -1 {
                    push_and_sort(&mut stack, (v+1, r-1, c))
                }
                if lies_in_grid(r as i32 + 1, c as i32) && seen[r+1][c] == -1 {
                    push_and_sort(&mut stack, (v+1, r+1, c))
                }
                seen[r][c] = v+1;
            },
            '-' => {
                if lies_in_grid(r as i32, c as i32 - 1) && seen[r][c-1] == -1 {
                    push_and_sort(&mut stack, (v+1, r, c-1))
                }
                if lies_in_grid(r as i32, c as i32 + 1) && seen[r][c+1] == -1 {
                    push_and_sort(&mut stack, (v+1, r, c+1))
                }
                seen[r][c] = v+1;
            },
            'L' => {
                if lies_in_grid(r as i32 - 1, c as i32) && seen[r-1][c] == -1 {
                    push_and_sort(&mut stack, (v+1, r-1, c))
                }
                if lies_in_grid(r as i32, c as i32 + 1) && seen[r][c+1] == -1 {
                    push_and_sort(&mut stack, (v+1, r, c+1))
                }
                seen[r][c] = v+1;
            },
            'J' => {
                if lies_in_grid(r as i32 - 1, c as i32) && seen[r-1][c] == -1 {
                    push_and_sort(&mut stack, (v+1, r-1, c))
                }
                if lies_in_grid(r as i32, c as i32 - 1) && seen[r][c-1] == -1 {
                    push_and_sort(&mut stack, (v+1, r, c-1))
                }
                seen[r][c] = v+1;
            },
            '7' => {
                if lies_in_grid(r as i32 + 1, c as i32) && seen[r+1][c] == -1 {
                    push_and_sort(&mut stack, (v+1, r+1, c))
                }
                if lies_in_grid(r as i32, c as i32 - 1) && seen[r][c-1] == -1 {
                    push_and_sort(&mut stack, (v+1, r, c-1))
                }
                seen[r][c] = v+1;
            },
            'F' => {
                if lies_in_grid(r as i32 + 1, c as i32) && seen[r+1][c] == -1 {
                    push_and_sort(&mut stack, (v+1, r+1, c))
                }
                if lies_in_grid(r as i32, c as i32 + 1) && seen[r][c+1] == -1 {
                    push_and_sort(&mut stack, (v+1, r, c+1))
                }
                seen[r][c] = v+1;
            },
            _ => ()
        }
        println!("Stack: {:?}", stack);
    }

    (Some(highest.to_string()), None)
}