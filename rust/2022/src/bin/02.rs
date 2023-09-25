use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/02.txt")
        .expect("Failed to read file");

    let strategy: Vec<(&str, &str)> = contents.lines()
        .map(|s| s.split(' ')
            .take(2)
            // .map(|slice| slice.to_string())
            .collect())
        .map(|v: Vec<&str>| (v[0].clone(), v[1].clone()))
        .collect();

    let mut score: i32 = 0;

    for game in &strategy {

        match game {
            ("A", "X") => score += 1 + 3,
            ("B", "X") => score += 1 + 0,
            ("C", "X") => score += 1 + 6,
            ("A", "Y") => score += 2 + 6,
            ("B", "Y") => score += 2 + 3,
            ("C", "Y") => score += 2 + 0,
            ("A", "Z") => score += 3 + 0,
            ("B", "Z") => score += 3 + 6,
            ("C", "Z") => score += 3 + 3,
            _ => println!("Should never happen!!")
        }
    }

    println!("Strategy's Score: {}", score);
}