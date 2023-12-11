pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let mut sum_a: i64 = 0;
    let mut sum_b: i64 = 0;
    for row in input {

        let mut readings: Vec<i64> = row.split_whitespace()
            .map(|reading| reading.parse::<i64>().unwrap())
            .collect();
        let mut diffs = vec![readings.clone()];

        while !diffs.last().unwrap().iter().all(|diff| *diff == 0i64) {
            let diff: Vec<i64> = diffs.last().clone().unwrap()
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            println!("Diff of {:?} is {:?}", diffs.last(), diff);
            diffs.push(diff);
        }

        for idx in (1..diffs.len()).rev() {
            let curr_diff = diffs[idx].clone();
            let next_base = diffs[idx - 1].last().unwrap().clone();
            let prev_base = diffs[idx - 1].first().unwrap().clone();
            diffs[idx - 1].push(curr_diff[curr_diff.len() - 1] + next_base);
            diffs[idx - 1].insert(0, prev_base - curr_diff[0])
        }
        println!("{:?} -> {:?}", readings, diffs[0]);
        sum_a += diffs[0].last().unwrap();
        sum_b += diffs[0].first().unwrap();
    }
    (Some(sum_a.to_string()),Some(sum_b.to_string()))
}