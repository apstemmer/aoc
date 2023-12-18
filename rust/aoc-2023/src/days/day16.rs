use std::cmp::max;
use std::collections::HashSet;

fn add_beam(beam: (i32, i32, char), beams: &mut Vec<(usize, usize, char)>, grid: &Vec<Vec<char>>) {
    if grid.len() > beam.0 as usize && beam.0 >= 0 && grid[0].len() > beam.1 as usize  && beam.1 >= 0 {
        beams.push((beam.0 as usize, beam.1 as usize, beam.2));
    }
}

fn count_energized(beam: (usize, usize, char), grid: &Vec<Vec<char>>) -> i32 {
    let mut seen: HashSet<(usize, usize, char)> = HashSet::new();
    let mut beams: Vec<(usize, usize, char)> = Vec::new();
    beams.push(beam);

    while beams.len() > 0 {
        let curr_beam = beams.pop().unwrap();
        if seen.contains(&curr_beam) {
            // println!("Already seen!");
            continue
        }

        match (curr_beam.0 as i32, curr_beam.1 as i32, curr_beam.2) {
            // Process beams on a /
            (r, c, '>') if grid[r as usize][c as usize] == '/' => add_beam((r-1, c, '^'), &mut beams, &grid),
            (r, c, 'v') if grid[r as usize][c as usize] == '/' => add_beam((r, c-1, '<'), &mut beams, &grid),
            (r, c, '<') if grid[r as usize][c as usize] == '/' => add_beam((r+1, c, 'v'), &mut beams, &grid),
            (r, c, '^') if grid[r as usize][c as usize] == '/' => add_beam((r, c+1, '>'), &mut beams, &grid),
            // Process beams on a \
            (r, c, '>') if grid[r as usize][c as usize] == '\\' => add_beam((r+1, c, 'v'), &mut beams, &grid),
            (r, c, 'v') if grid[r as usize][c as usize] == '\\' => add_beam((r, c+1, '>'), &mut beams, &grid),
            (r, c, '<') if grid[r as usize][c as usize] == '\\' => add_beam((r-1, c, '^'), &mut beams, &grid),
            (r, c, '^') if grid[r as usize][c as usize] == '\\' => add_beam((r, c-1, '<'), &mut beams, &grid),
            // Process beams on a .
            (r, c, '>') if grid[r as usize][c as usize] == '.' => add_beam((r, c+1, '>'), &mut beams, &grid),
            (r, c, 'v') if grid[r as usize][c as usize] == '.' => add_beam((r+1, c, 'v'), &mut beams, &grid),
            (r, c, '<') if grid[r as usize][c as usize] == '.' => add_beam((r, c-1, '<'), &mut beams, &grid),
            (r, c, '^') if grid[r as usize][c as usize] == '.' => add_beam((r-1, c, '^'), &mut beams, &grid),
            // Process beams on a -
            (r, c, '>') if grid[r as usize][c as usize] == '-' => add_beam((r, c+1, '>'), &mut beams, &grid),
            (r, c, 'v') if grid[r as usize][c as usize] == '-' => {
                add_beam((r, c+1, '>'), &mut beams, &grid);
                add_beam((r, c-1, '<'), &mut beams, &grid);
            },
            (r, c, '<') if grid[r as usize][c as usize] == '-' => add_beam((r, c-1, '<'), &mut beams, &grid),
            (r, c, '^') if grid[r as usize][c as usize] == '-' => {
                add_beam((r, c+1, '>'), &mut beams, &grid);
                add_beam((r, c-1, '<'), &mut beams, &grid);
            },
            // Process beams on a |
            (r, c, '>') if grid[r as usize][c as usize] == '|' => {
                add_beam((r-1, c, '^'), &mut beams, &grid);
                add_beam((r+1, c, 'v'), &mut beams, &grid);
            },
            (r, c, 'v') if grid[r as usize][c as usize] == '|' => add_beam((r+1, c, 'v'), &mut beams, &grid),
            (r, c, '<') if grid[r as usize][c as usize] == '|' => {
                add_beam((r-1, c, '^'), &mut beams, &grid);
                add_beam((r+1, c, 'v'), &mut beams, &grid);
            },
            (r, c, '^') if grid[r as usize][c as usize] == '|' => add_beam((r-1, c, '^'), &mut beams, &grid),
            _ => panic!("Beam does not match any rule!")
        }
        seen.insert(curr_beam.clone());
    }

    let energized: HashSet<_> = seen.into_iter()
        .map(|(r, c, d)| (r, c))
        .collect();

    energized.len() as i32
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let grid:Vec<Vec<char>> = input.iter().map(|r| r.chars().collect()).collect();
    let energized_a: i32 = count_energized((0,0,'>'), &grid);

    let mut max_energized = 0;
    for r in 0..grid.len() {
        max_energized = max(max_energized, count_energized((r, 0, '>'), &grid));
        max_energized = max(max_energized, count_energized((r, grid[0].len() - 1, '<'), &grid));
    }
    for c in 0..grid[0].len() {
        max_energized = max(max_energized, count_energized((0, c, 'v'), &grid));
        max_energized = max(max_energized, count_energized((grid.len() - 1, c, '^'), &grid));
    }

    (Some(energized_a.to_string()), Some(max_energized.to_string()))
}