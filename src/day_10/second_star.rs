use std::fs;
use ndarray::Array2;

use super::first_star::Coords;
use super::first_star::get_valid_neighbors;

// Problem link : https://adventofcode.com/2024/day/10#part2
// (Only available after solving the first part)

// This problem is the same as the first one, but using vec instead of hashset for keeping the count of leaves

pub fn main() {
    let file_content = fs::read_to_string("./src/day_10/input")
        .expect("Error reading the file.");

    let lines: Vec<&str> = file_content.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut matrix = Array2::<u8>::zeros((rows, cols));

    // Creating the matrix of the topographic map
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            matrix[[i, j]] = c.to_digit(10).unwrap() as u8;
        }
    }

    let mut score: usize = 0;

    // Initialising the graph
    for ((i, j), value) in matrix.indexed_iter() {
        if *value == 0 {
            let mut leaves: Vec<Coords> = vec![(i, j)];

            for value in 1..10 as u8 {
                let mut new_leaves: Vec<Coords> = vec![];

                for (li, lj) in leaves.iter() {
                    let neighbors = get_valid_neighbors(&matrix, (*li, *lj), value);

                    for neighbor in neighbors.iter() {
                        new_leaves.push(*neighbor);
                    }
                }

                leaves = new_leaves;
            }

            score += leaves.len();
        }
    }

    println!("{}", score);
}