
pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let times: Vec<i32> = input[0].split(":").collect::<Vec<_>>()[1]
        .split_whitespace()
        .map(|t|t.parse::<i32>()
        .unwrap()).collect();

    let distances: Vec<i32> = input[1].split(":").collect::<Vec<_>>()[1]
        .split_whitespace()
        .map(|t|t.parse::<i32>()
        .unwrap()).collect();

    let time_b = input[0].split(":").collect::<Vec<_>>()[1].chars()
        .filter(|c| !c.is_whitespace()).collect::<String>().parse::<i64>().unwrap();

    let dist_b = input[1].split(":").collect::<Vec<_>>()[1].chars()
        .filter(|c| !c.is_whitespace()).collect::<String>().parse::<i64>().unwrap();

    println!("B: {}  {}", time_b, dist_b);

    let mut sum_a = 1;
    for race in 0..times.len() {
        let mut wins = 0;
        for hold in 0..times[race] + 1 {
            if hold * (times[race] - hold) > distances[race] {
                wins += 1;
            }
        }
        println!("Wins {}", wins);
        sum_a *= wins;
    }
    let mut sum_b = 0;
    for hold in 0..time_b+1 {
        if hold * (time_b - hold) > dist_b {
            sum_b += 1;
        }
    }

    println!("{:?}, {:?}", times, distances);
    (Some(sum_a.to_string()), Some(sum_b.to_string()))
}