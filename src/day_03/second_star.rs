use regex::Regex;
use std::fs;

// Problem link : https://adventofcode.com/2024/day/3#part2
// (Only available after solving the first part)

pub fn find_closest_index(list: Vec<usize>, target: usize) -> usize {
    for (i, index) in list.iter().enumerate() {
        if *index >= target {
            if i == 0 {
                return 0;
            } else {
                return list[i-1];
            }
        }
    }

    list[list.len() - 1]
}

pub fn main() {
    let file_content = fs::read_to_string("./src/day_03/input")
        .expect("Error reading the file.");

    // Getting the positions of do() and don't() in the file
    let mut dos_index: Vec<usize> = Regex::new(r"do\(\)").unwrap()
        .find_iter(&file_content)
        .map(|e| e.start())
        .collect();

    // At the beginning, expressions are "enabled"
    dos_index.insert(0, 0);

    let donts_index: Vec<usize> = Regex::new(r"don\'t\(\)").unwrap()
        .find_iter(&file_content)
        .map(|e| e.start())
        .collect();

    let re_mul = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let expressions: Vec<(usize, &str)> = re_mul.find_iter(&file_content)
        .map(|e| (e.start(), e.as_str()))
        .collect();

    let mut result = 0;

    for (i, expression) in expressions.iter() {
        let closest_do = find_closest_index(dos_index.clone(), *i);
        let closest_dont = find_closest_index(donts_index.clone(), *i);

        if closest_do >= closest_dont {
            let values: Vec<i32> = Regex::new(r"\d{1,3}").unwrap()
                .find_iter(&expression)
                .map(|e| e.as_str().parse().unwrap())
                .collect();

            result += values.iter().product::<i32>();
        }
    }

    println!("{}", result);
}