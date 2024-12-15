use std::fs;

// Problem link : https://adventofcode.com/2024/day/14

pub type Coords = (u8, u8);
pub type Speed = (i8, i8);
pub type Robot = (Coords, Speed);

pub fn parse_robots(file_content: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];

    for line in file_content.lines() {
        let (coords, speed) = line.split_once(" ")
            .expect("Invalid robot.");

        let (_, actual_coords) = coords.split_once("=")
            .expect("Invalid coordinates.");

        let (_, actual_speed) = speed.split_once("=")
            .expect("Invalid speed.");

        let (x, y) = actual_coords.split_once(",")
            .map(
                |(x, y)| (
                    x.parse::<u8>().expect("Invalid x."),
                    y.parse::<u8>().expect("Invalid y.")
                )
            )
            .expect("Invalid x and y coordinates.");

        let (xs, ys) = actual_speed.split_once(",")
            .map(
                |(xs, ys)| (
                    xs.parse::<i8>().expect("Invalid xs."),
                    ys.parse::<i8>().expect("Invalid ys.")
                )
            )
            .expect("Invalid x and y speed.");

        robots.push(((x, y), (xs, ys)));
    }

    robots
}

fn final_quadrant(robot: Robot, grid_shape: Coords, turns: i8) -> Option<usize> {
    let new_x: i32 = robot.0.0 as i32 + robot.1.0 as i32 * turns as i32;
    let new_y: i32 = robot.0.1 as i32 + robot.1.1 as i32 * turns as i32;

    let mut final_x = new_x % grid_shape.0 as i32;
    let mut final_y = new_y % grid_shape.1 as i32;

    if final_x < 0 {
        final_x = grid_shape.0 as i32 + final_x;
    }

    if final_y < 0 {
        final_y = grid_shape.1 as i32 + final_y;
    }

    // Quadrant ID
    // 0 1
    // 2 3
    let mut quadrant = 0;

    if final_x == grid_shape.0 as i32 / 2 {
        return None;
    }

    if final_y == grid_shape.1 as i32 / 2 {
        return None;
    }

    if final_x > grid_shape.0 as i32 / 2 {
        quadrant += 2;
    }

    if final_y > grid_shape.1 as i32 / 2 {
        quadrant += 1;
    }

    Some(quadrant)
}

pub fn main() {
    let file_content = fs::read_to_string("./src/day_14/input")
        .expect("Error reading the file.");

    let robots: Vec<Robot> = parse_robots(&file_content);

    let grid_shape: Coords = (101, 103);

    // Quadrant ID
    // 0 1
    // 2 3
    let mut quadrants: Vec<u32> = vec![0, 0, 0, 0];

    for robot in robots.iter() {
        match final_quadrant(*robot, grid_shape, 100) {
            Some(quadrant) => quadrants[quadrant] += 1,
            None                  => continue 
        }
    }
    println!("{:?}", quadrants);
    let safety_factor: u32 = quadrants.iter().fold(1, |acc, q| acc * q);

    println!("{}", safety_factor);
}