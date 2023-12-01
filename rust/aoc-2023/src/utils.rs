use std::fs;

pub fn read_input(day: u8) -> Vec<String> {
    let filename = format!("inputs/{:02}.txt", day);
    let contents: String = fs::read_to_string(filename).expect("Should read file as string");

    let lines : Vec<String> = contents.lines()
        .map(|s| s.to_string())
        .collect();

    lines
}