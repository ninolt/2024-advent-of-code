use std::fs;

// Problem link : https://adventofcode.com/2024/day/2

fn is_report_safe(levels: Vec<i32>) -> bool {
    // true is increasing, false is decreasing
    let is_increasing = levels[levels.len()-1] - levels[0] > 0; 

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i-1];

        if is_increasing && (diff < 1 || diff > 3) {
            return false;
        } else if !is_increasing && (diff > -1 || diff < -3) {
            return false;
        }
    }

    true
}

pub fn main() {
    let file_content = fs::read_to_string("./src/day_02/input")
        .expect("Error reading the file.");

    let mut total_safe_reports = 0;

    for line in file_content.lines() {
        let report: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();

        if is_report_safe(report) {
            total_safe_reports += 1;
        }
    }

    println!("{}", total_safe_reports);
}