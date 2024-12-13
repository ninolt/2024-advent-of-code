use std::fs;
use ndarray::Array2;
use std::collections::HashSet;

// Problem link : https://adventofcode.com/2024/day/10

pub type Coords = (usize, usize);

pub fn get_valid_neighbors(matrix: &Array2<u8>, coords: Coords, to_check: u8) -> Vec<Coords> {
    let mut valid_neighbors: Vec<Coords> = vec![];
    let shape = matrix.shape();

    if coords.0 > 0 && matrix[[coords.0 - 1, coords.1]] == to_check {
        valid_neighbors.push((coords.0 - 1, coords.1));
    }

    if coords.0 < shape[0] - 1 && matrix[[coords.0 + 1, coords.1]] == to_check {
        valid_neighbors.push((coords.0 + 1, coords.1));
    }

    if coords.1 > 0 && matrix[[coords.0, coords.1 - 1]] == to_check {
        valid_neighbors.push((coords.0, coords.1 - 1));
    }

    if coords.1 < shape[1] - 1 && matrix[[coords.0, coords.1 + 1]] == to_check {
        valid_neighbors.push((coords.0, coords.1 + 1));
    }

    valid_neighbors
}

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
            let mut leaves: HashSet<Coords> = HashSet::new();
            leaves.insert((i, j));

            for value in 1..10 as u8 {
                let mut new_leaves: HashSet<Coords> = HashSet::new();

                for (li, lj) in leaves.iter() {
                    let neighbors = get_valid_neighbors(&matrix, (*li, *lj), value);

                    for neighbor in neighbors.iter() {
                        new_leaves.insert(*neighbor);
                    }
                }

                leaves = new_leaves;
            }

            score += leaves.len();
        }
    }

    println!("{}", score);
}