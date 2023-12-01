use clap::{Parser};
mod utils;
mod days;

use days::*;

#[derive(Parser, Debug)]
struct Args {

    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();

    println!("Working on day: {}", args.day);
    let input = utils::read_input(args.day);

    let result: (Option<String>, Option<String>) = match args.day {
        1 => day01::execute(input),
        _ => (None, None)
    };

    match result.0 {
        Some(T) => println!("Result for part A: {}", T),
        None => println!("No result for part A.")
    }

    match result.1 {
        Some(T) => println!("Result for part B: {}", T),
        None => println!("No result for part B.")
    }
}
