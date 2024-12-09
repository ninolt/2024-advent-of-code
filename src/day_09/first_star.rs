use std::fs;
use std::cmp;

// Problem link : https://adventofcode.com/2024/day/9

pub fn main() {
    let file_content = fs::read_to_string("./src/day_09/input")
        .expect("Error reading the file.");

    let list: Vec<u8> = file_content.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut checksum: usize = 0;
    // Moving index for calculating the checksum
    let mut checksum_index: usize = 0;

    let mut space_index: usize = 1;
    // Padding for keeping in memory the remaining space in the free space
    let mut space_remaining: u8 = list[space_index];

    let mut end_index: usize = list.len() - (list.len() % 2);
    // Padding for keeping in memory the remaining size of the block to move
    let mut end_remaining: u8 = list[end_index];

    let mut new_space_index: bool = true;

    while space_index < end_index {
        if new_space_index {
            // Adding the non-moving blocks to the checksum
            for _ in 0..list[space_index - 1] {
                checksum += checksum_index * ((space_index - 1) / 2);
                checksum_index += 1;
            }

            new_space_index = false;
        }

        // Finishing either the padding of free space or the padding of block to move
        for _ in 0..cmp::min(space_remaining, end_remaining) {
            checksum += checksum_index * (end_index / 2);
            
            checksum_index += 1;
            space_remaining -= 1;
            end_remaining -= 1;
        }
        
        // Changing to next free space if needed
        if space_remaining == 0 {
            space_index += 2;
            space_remaining = list[space_index];
            new_space_index = true;
        }

        // Changing to next block to move if needed
        if end_remaining == 0 {
            end_index -= 2;
            end_remaining = list[end_index];
        }
    }

    // For what's left to add to the checksum
    for _ in 0..end_remaining {
        checksum += checksum_index * (end_index / 2);
        checksum_index += 1;
        end_remaining -= 1;
    }

    println!("{}", checksum);
}