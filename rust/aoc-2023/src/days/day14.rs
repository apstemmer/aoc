pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let grid: Vec<Vec<char>> = input.iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let mut sum_a = 0;
    let max_weight = input.len();
    for col in 0..input[0].len() {
        let mut weight = max_weight;
        let mut line = Vec::new();
        for row in 0..input.len() {
            line.push(grid[row][col]);
            match grid[row][col] {
                '#' => weight = max_weight - (row + 1),
                'O' => {
                    sum_a += weight;
                    weight -= 1;
                }
                _ => ()
            }
        }
    }
    (Some(sum_a.to_string()), None)
}