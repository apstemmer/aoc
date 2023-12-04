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
    let args = Args::parse();

    println!("Working on day: {}", args.day);
    let input = utils::read_input(args.day);

    let result: (Option<String>, Option<String>) = match args.day {
        1 => day01::execute(input),
        2 => day02::execute(input),
        3 => day03::execute(input),
        4 => day04::execute(input),
        5 => day05::execute(input),
        6 => day06::execute(input),
        7 => day07::execute(input),
        _ => (None, None)
    };

    match result.0 {
        Some(res_a) => println!("Result for part A: {}", res_a),
        None => println!("No result for part A.")
    }

    match result.1 {
        Some(res_b) => println!("Result for part B: {}", res_b),
        None => println!("No result for part B.")
    }
}
