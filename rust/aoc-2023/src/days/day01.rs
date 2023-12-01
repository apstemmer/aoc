use std::{i32};
use std::collections::HashMap;

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {

    let mut sum_a: i32 = 0;
    let mut sum_b: i32 = 0;
    for calibration in &input {
        let stripped: String = calibration.chars().filter(|c| c.is_numeric()).collect();
        let cal_val = format!("{}{}", stripped.chars().next().unwrap(), stripped.chars().last().unwrap());

        if let Ok(num) = cal_val.parse::<i32>(){
            sum_a += num;
            println!("A: {} -> {} -> {} -> {}", calibration, stripped, cal_val, sum_a);
        };

        let mapping: HashMap<&str, &str> = vec![
            ("0", "0"), ("1", "1"), ("2", "2"),
            ("3", "3"), ("4", "4"), ("5", "5"),
            ("6", "6"), ("7", "7"), ("8", "8"), ("9", "9"),
            ("zero", "0"), ("one", "1"), ("two", "2"),
            ("three", "3"), ("four", "4"), ("five", "5"),
            ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")
        ].into_iter().collect();

        let substrings: Vec<&str> = mapping.keys().cloned().collect();

        let mut left_idx = i32::MAX;
        let mut left_match = None;
        let mut right_idx = i32::MIN;
        let mut right_match = None;

        for str in substrings {
            match calibration.find(str) {
                Some(pos) if (pos as i32) < left_idx => { left_idx = pos as i32; left_match = Some(str) }
                _ => ()
            }

            match calibration.rfind(str) {
                Some(pos) if (pos as i32) > right_idx => {right_idx = pos as i32; right_match = Some(str)}
                _ => ()
            }
        }

        if let (Some(left), Some(right)) = (left_match, right_match) {
            let cal_val_b :i32 = format!("{}{}", mapping[left], mapping[right]).parse::<i32>().unwrap().abs();
            sum_b += cal_val_b;
            println!("B: {} -> {} -> {}", calibration, cal_val_b, sum_a);
        };
    }
    (Some(sum_a.to_string()), Some(sum_b.to_string()))
}