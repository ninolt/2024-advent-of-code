use std::fs;

// Problem link : https://adventofcode.com/2024/day/1

/* After sorting the two lists, this problem is just the Manhattan distance
 * between two vectors.
 */

fn qsort<I32: Ord>(mut list: Vec<I32>) -> Vec<I32> {
    if list.len() <= 1 {
        return list;
    }

    let pivot = list.remove(0);
    let mut left: Vec<I32> = vec![];
    let mut right: Vec<I32> = vec![];

    for value in list {
        if value <= pivot {
            left.push(value);
        } else {
            right.push(value);
        }
    }

    let mut sorted_left = qsort(left);
    let mut sorted_right = qsort(right);

    sorted_left.push(pivot);
    sorted_left.append(&mut sorted_right);

    sorted_left
}

fn manhattan_dist(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let mut sum = 0;

    for (a, b) in list1.iter().zip(list2.iter()) {
        sum += (a - b).abs();
    }

    sum
}

// Public function so the second star of the day can re-use it
pub fn get_vec<I32>() -> (Vec<i32>, Vec<i32>) {
    let file_content = fs::read_to_string("./src/day_01/input")
        .expect("Error reading the file.");

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in file_content.lines() {
        if let Some((first_value, second_value)) = line.split_once("   ") {
            list1.push(first_value.parse().unwrap());
            list2.push(second_value.parse().unwrap());
        } else {
            panic!("The file is invalid: one line does not have a valid separator.");
        };
    }

    (list1, list2)
}

pub fn main() {
    let (mut list1, mut list2) = get_vec::<i32>();

    list1 = qsort(list1);
    list2 = qsort(list2);

    println!("{}", manhattan_dist(list1, list2));
}
