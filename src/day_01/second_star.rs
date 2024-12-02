use std::collections::HashMap;

use super::first_star::get_vec;

// Problem link : https://adventofcode.com/2024/day/1#part2
// (Only available after solving the first part)

pub fn main() {
    // Using the same method as the first star to get the vectors
    let (list1, list2) = get_vec::<i32>();

    let mut list2_entries: HashMap<i32, i32> = HashMap::new();

    // Creating the numbrer of entries for the second list
    for value in list2.iter() {
        *list2_entries.entry(*value).or_insert(0) += 1;
    }

    // Calculating the "similarity score"
    let mut similarity_score = 0;

    for value in list1 {
        similarity_score += value * list2_entries.get(&value).unwrap_or(&0);
    }

    println!("{}", similarity_score);
}