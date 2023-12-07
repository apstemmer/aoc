use std::cmp::Ordering;
use std::collections::HashMap;


fn compare_hand(hand1 : &(String, i32), hand2 : &(String, i32)) -> Ordering {
    let cards:HashMap<char, i32> = vec![
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ].into_iter().collect();
    for i in 0..hand1.0.len() {
        let c1 = hand1.0.chars().nth(i).unwrap();
        let c2 = hand2.0.chars().nth(i).unwrap();
        if cards.get(&c1) < cards.get(&c2) {
            return Ordering::Greater;
        } else if cards.get(&c1) > cards.get(&c2) {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut types: Vec<Vec<(String, i32)>> = vec![Vec::new(); 7];

    for row in &input {
        let mut char_counts:HashMap<char, i32> = HashMap::new();
        let (hand_, bid_) = row.split_at(5);
        let bid = bid_.strip_prefix(" ").unwrap().parse::<i32>().unwrap();

        let hand = (hand_.to_string(), bid);
        for c in hand.0.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }
        let mut top: Vec<i32> = char_counts.values().cloned().collect();
        top.sort_by(|a, b| b.cmp(a));

        match (top[0], if top.len() > 1 { top[1] } else { 0 }) {
            (5, _) => types[0].push(hand.clone()),
            (4, _) => types[1].push(hand.clone()),
            (3, 2) => types[2].push(hand.clone()),
            (3, _) => types[3].push(hand.clone()),
            (2, 2) => types[4].push(hand.clone()),
            (2, _) => types[5].push(hand.clone()),
            (1, _) => types[6].push(hand.clone()),
            _ => panic!("Could not process row!")
        }

        println!("Hand: {} -> {:?}", hand.0, top);
    }
    let mut pos = input.len();
    let mut sum_a:i64 = 0;
    for hand_type in &types {
        let mut curr_type = hand_type.clone();
        curr_type.sort_by(compare_hand);
        println!("{:?}\n\n", curr_type);
        for (_, bid) in curr_type {
            sum_a += (bid as i64) * (pos as i64);
            pos -= 1;
        }
    }

    (Some(sum_a.to_string()), None)
}