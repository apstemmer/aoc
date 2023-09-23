use std::fs;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("src/input.txt").expect("Should read to file!");
    
    let lines: Vec<String> = contents.lines()
        .map(|s| s.to_string())
        .collect();

    let mut top = 0;
    let mut curr: i32 = 0;

    for s in &lines {
        println!("s {}, c {}, t {}", s, curr, top);
        if s.is_empty() {
            println!("Empty Line!");
            if curr > top {
                top = curr;
            }
            curr = 0;
        } else {
            curr += s.parse::<i32>().unwrap();    
        }
        
    }
    println!("Result: {}", top);
}
