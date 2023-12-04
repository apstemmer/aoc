use std::collections::HashSet;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {

    let mut sum_a = 0;

    for card in input {
        let mut split_card: Vec<&str> = card.split(|c| c == ':' || c == '|').collect();
        let mut winners: HashSet<i32> = split_card[1].split_whitespace()
            .filter_map(|num_str| num_str.parse().ok())
            .collect();
        let mut numbers: HashSet<i32> = split_card[2].split_whitespace()
            .filter_map(|num_str| num_str.parse().ok())
            .collect();
        let mut intersection: Vec<i32> = winners.intersection(&numbers).cloned().collect();

        match intersection.len() {
            p if p > 0 => sum_a += 2_i32.pow((intersection.len() - 1) as u32),
            _ => ()
        }
    }

    (Some(sum_a.to_string()), None)
}