pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let sequences: Vec<Vec<i64>> = input
        .iter()
        .flat_map(|s| s.split(',')
            .map(|substring| substring.chars().map(|c| c as i64).collect())
        )
        .collect();
    let mut sum_a:i64 = 0;
    for sequence in sequences {
        let mut hash:i64 = 0;
        for val in sequence.iter() {
            hash += val;
            hash = hash * 17;
            hash = hash % 256;
        }
        sum_a += hash
    }
    (Some(sum_a.to_string()), None)
}