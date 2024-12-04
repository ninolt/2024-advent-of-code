use regex::Regex;
use std::fs;

// Problem link : https://adventofcode.com/2024/day/3

pub fn main() {
    let file_content = fs::read_to_string("./src/day_03/input")
        .expect("Error reading the file.");

    let re_mul = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let expressions: Vec<_> = re_mul.find_iter(&file_content)
        .map(|e| e.as_str())
        .collect();

    let mut result = 0;

    for expression in expressions.iter() {
        let values: Vec<i32> = Regex::new(r"\d{1,3}").unwrap()
            .find_iter(&expression)
            .map(|e| e.as_str().parse().unwrap())
            .collect();

        result += values.iter().product::<i32>();
    }

    println!("{}", result);
}