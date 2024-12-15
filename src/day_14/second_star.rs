use std::fs;
use ndarray::Array2;
use std::collections::HashMap;

use super::first_star::Coords;
use super::first_star::Robot;
use super::first_star::parse_robots;

// Problem link : https://adventofcode.com/2024/day/14#part2
// (Only available after solving the first part)

fn euclidian_distance(p1: Coords, p2: Coords) -> f32 {
    f32::sqrt(((p1.0 as i32 - p2.0 as i32).pow(2) + (p1.1 as i32 - p2.1 as i32).pow(2)) as f32)
}

fn get_new_position(robot: Robot, grid_shape: Coords) -> Coords {
    let new_x = robot.0.0 as i32 + robot.1.0 as i32;
    let new_y  = robot.0.1 as i32 + robot.1.1 as i32;

    let mut final_x = new_x % grid_shape.0 as i32;
    let mut final_y = new_y % grid_shape.1 as i32;

    if final_x < 0 {
        final_x = grid_shape.0 as i32 + final_x;
    }

    if final_y < 0 {
        final_y = grid_shape.1 as i32 + final_y;
    }

    (final_x as u8, final_y as u8)
}

pub fn main() {
    let file_content = fs::read_to_string("./src/day_14/input")
        .expect("Error reading the file.");

    let mut robots: Vec<Robot> = parse_robots(&file_content);

    let grid_shape: Coords = (101, 103);
    let grid_center = (grid_shape.0 / 2, grid_shape.1 / 2);

    let mut avg_distance: HashMap<usize, f32> = HashMap::new();

    for i in 1..10000 {
        let mut grid: Array2<u8> = Array2::<u8>::zeros((grid_shape.0 as usize, grid_shape.1 as usize));
        let mut acc_dist: f32 = 0.0;

        for robot in robots.iter_mut() {
            let robot_coords = get_new_position(*robot, grid_shape);

            // Updating the coords of the robot for the next second
            robot.0 = robot_coords;

            grid[(robot_coords.0 as usize, robot_coords.1 as usize)] += 1;

            acc_dist += euclidian_distance(robot.0, grid_center);
        }

        // Calculating the variance for each second
        avg_distance.insert(i, acc_dist / robots.len() as f32);
    }

    let min_dist_index = avg_distance.iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(k, _)| k);

    match min_dist_index {
        Some(second) => println!("{}", second),
        None                 => println!("There is an error finding the smallest distance from the center.")
    }
}