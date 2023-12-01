pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    // TODO: Implement AOC

    let mut sum_a: i32 = 0;
    for calibration in &input {
        let stripped: String = calibration.chars().filter(|c| c.is_numeric()).collect();
        let cal_val = format!("{}{}", stripped.chars().next().unwrap(), stripped.chars().last().unwrap());

        if let Ok(num) = cal_val.parse::<i32>(){
            sum_a += num;
            println!("{} -> {} -> {} -> {}", calibration, stripped, cal_val, sum_a);
        };
    }
    (Some(sum_a.to_string()), None)
}