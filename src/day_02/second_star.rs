use std::fs;

// Problem link : https://adventofcode.com/2024/day/2#part2
// (Only available after solving the first part)

fn is_report_safe(levels: Vec<i32>, mut removable_bad_level: i32) -> bool {
    // true is increasing, false is decreasing
    let is_increasing = levels[levels.len()-1] - levels[0] > 0; 

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i-1];

        if (is_increasing && (diff < 1 || diff > 3)) ||
            (!is_increasing && (diff > -1 || diff < -3)) {
            
            if removable_bad_level == 0 {
                return false;
            } else {
                removable_bad_level -= 1;

                let first_ignore: Vec<i32> = levels.iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_,x)| x)
                    .copied()
                    .collect();

                let second_ignore: Vec<i32> = levels.iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i-1)
                    .map(|(_,x)| x)
                    .copied()
                    .collect();

                return is_report_safe(first_ignore, removable_bad_level) ||
                    is_report_safe(second_ignore, removable_bad_level);
            }
        }
    }

    true
}

pub fn main() {
    let file_content = fs::read_to_string("input")
        .expect("Error reading the file.");

    let mut total_safe_reports = 0;

    for line in file_content.lines() {
        let report: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();

        if is_report_safe(report, 1) {
            total_safe_reports += 1;
        }
    }

    println!("{}", total_safe_reports);
}