use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/01.txt").expect("Should read to file!");
    
    let lines: Vec<String> = contents.lines()
        .map(|s| s.to_string())
        .collect();

    let mut top = 0;
    let mut curr: i32 = 0;

    let mut elves: Vec<i32> = Vec::new();

    for s in &lines {
        println!("s {}, c {}, t {}", s, curr, top);
        if s.is_empty() {
            println!("Empty Line!");
            elves.push(curr);
            curr = 0;
        } else {
            curr += s.parse::<i32>().unwrap();    
        }
        
    }
    elves.sort();
    elves.reverse();

    println!("Result: \na: {} \nb: {}", elves[0], elves[0] + elves[1] + elves[2]);
}
