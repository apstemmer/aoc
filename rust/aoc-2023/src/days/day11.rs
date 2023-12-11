
fn distort_galaxies(galaxies:Vec<(usize, usize)>, rows_to_expand:Vec<usize>, cols_to_expand:Vec<usize>) -> Vec<(usize, usize)> {
    let mut distorted_galaxies = Vec::new();
    for galaxy in galaxies {
        let mut rows:usize = 0;
        let mut cols:usize = 0;
        while rows < rows_to_expand.len() && rows_to_expand[rows] < galaxy.0  {
            rows += 1;
        }
        while cols < cols_to_expand.len() && cols_to_expand[cols] < galaxy.1 {
            cols += 1;
        }
        distorted_galaxies.push((galaxy.0 + rows, galaxy.1 + cols))
    }
    distorted_galaxies
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let image: Vec<Vec<char>> = input.iter().map(|row| row.chars().collect()).collect();
    let mut rows_to_expand: Vec<usize> = Vec::new();
    let mut cols_to_expand: Vec<usize>= Vec::new();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (r, row) in image.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            rows_to_expand.push(r);
        } else {
            for (c, value) in row.iter().enumerate() {
                if *value == '#' {
                    galaxies.push((r, c));
                }
            }
        }
    }
    for col in 0..image[0].len() {
        let mut column:Vec<char> = Vec::new();
        for row in 0..image.len() {
            column.push(image[row][col])
        }
        println!("Col {}: {:?}", col, column);
        if column.iter().all(|c| *c == '.') {
            cols_to_expand.push(col);
        }
    }
    println!("{:?} {:?}", rows_to_expand, cols_to_expand);
    println!("{:?}", galaxies);
    let distorted_galaxies = distort_galaxies(galaxies, rows_to_expand, cols_to_expand);
    println!("{:?}", distorted_galaxies);
    let mut sum_a:i64 = 0;
    for src_num in 0..distorted_galaxies.len() {
        for dst_num in 0..distorted_galaxies.len() {
            let src = distorted_galaxies[src_num];
            let dst = distorted_galaxies[dst_num];
            sum_a += (src.0 as i64 - dst.0 as i64).abs() + (src.1 as i64 - dst.1 as i64).abs();
        }
    }
    println!("Total Distance {}", sum_a/2);
    (Some((sum_a / 2).to_string()), None)
}