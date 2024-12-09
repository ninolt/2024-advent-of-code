use std::fs;

// Problem link : https://adventofcode.com/2024/day/9#part2
// (Only available after solving the first part)

pub fn main() {
    let file_content = fs::read_to_string("./src/day_09/input")
        .expect("Error reading the file.");

    let list: Vec<usize> = file_content.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    // Saving changing available place for every space in the disk
    let mut available_places: Vec<usize> = list.iter()
        .enumerate()
        .filter(|(i, _)| i%2 == 1)
        .map(|(_, v)| *v)
        .collect();

    let mut cumul: Vec<usize> = vec![0];

    // Cumulative sum for every block, representing the index for the final checksum calculation
    for v in list.iter() {
        cumul.push(*v + cumul[cumul.len() - 1]);
    }

    let mut checksum: usize = 0;

    let mut end_index: usize = list.len() - (list.len() % 2);

    while end_index > 0 {
        let size: usize = list[end_index];
        let mut found_free_space: bool = false;

        for (i, space) in available_places.iter().enumerate() {
            // Index of the corresponding available space
            let actual_index: usize = i*2 + 1;

            if actual_index > end_index {
                break;
            }

            if *space >= size {
                for j in 0..size {
                    // Score of the new position of the block in a free space on its left
                    checksum += (cumul[actual_index] + (list[actual_index] - space + j)) * (end_index / 2);
                }
    
                available_places[i] -= size;

                found_free_space = true;

                break;
            }
        }

        // If the block didn't move, calculating its score from its current position
        if !found_free_space {
            for i in 0..size {
                checksum += (cumul[end_index] + i) * (end_index / 2);
            }
        }

        end_index -= 2;
    }

    println!("{}", checksum);
}