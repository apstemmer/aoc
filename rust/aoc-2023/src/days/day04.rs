use std::collections::HashSet;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {

    let mut sum_a = 0;
    let mut memo = vec![1; input.len()];
    let mut curr_idx = 0;

    for card in input.iter().rev() {
        let split_card: Vec<&str> = card.split(|c| c == ':' || c == '|').collect();
        let winners: HashSet<i32> = split_card[1].split_whitespace()
            .filter_map(|num_str| num_str.parse().ok())
            .collect();
        let numbers: HashSet<i32> = split_card[2].split_whitespace()
            .filter_map(|num_str| num_str.parse().ok())
            .collect();
        let intersection: Vec<i32> = winners.intersection(&numbers).cloned().collect();

        match intersection.len() {
            p if p > 0 => {
                sum_a += 2_i32.pow((intersection.len() - 1) as u32);
                let mut card_count = 0;
                for copy in 1..= p {
                    card_count += memo[curr_idx - copy]
                }
                memo[curr_idx] = 1 + card_count;
            },
            _ => ()
        }
        curr_idx += 1;
    }

    (Some(sum_a.to_string()), Some(memo.iter().sum::<i32>().to_string()))
}