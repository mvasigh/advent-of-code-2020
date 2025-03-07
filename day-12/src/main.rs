use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn get_input_reader() -> io::Result<BufReader<File>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn part_one() -> i32 {
    let input_reader = get_input_reader().expect("Could not read input data");

    let mut dx = 0;
    let mut dy = 0;
    let mut direction = 0;
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)]; // E, S, W, N

    for line_result in input_reader.lines() {
        let line = line_result.expect("Could not read line");
        let (command, raw_value) = line.split_at(1);
        let value: i32 = raw_value.parse().unwrap();

        match command {
            "N" => dy += value,
            "E" => dx += value,
            "S" => dy -= value,
            "W" => dx -= value,
            "R" => {
                let direction_change = value / 90;
                direction = (direction + direction_change) % 4;
            }
            "L" => {
                let direction_change = -1 * value / 90;
                direction = if direction + direction_change >= 0 {
                    (direction + direction_change) % 4
                } else {
                    4 + (direction + direction_change)
                };
            }
            "F" => {
                let (x, y) = directions[direction as usize];
                dx += x * value;
                dy += y * value;
            }
            &_ => {} // default case
        }
    }
    dx.abs() + dy.abs()
}

fn rotate(mut pt: (i32, i32), rotations: i32) -> (i32, i32) {
    let clockwise = rotations > 0;
    let mut iterations = rotations.abs();

    while iterations > 0 {
        pt = if clockwise {
            (pt.1, pt.0 * -1)
        } else {
            (pt.1 * -1, pt.0)
        };
        iterations -= 1;
    }
    pt
}

fn part_two() -> i32 {
    let input_reader = get_input_reader().expect("Could not read input data");

    let mut dx = 0;
    let mut dy = 0;
    let mut waypoint = (10, 1); // (x, y)

    for line_result in input_reader.lines() {
        let line = line_result.expect("Could not read line");
        let (command, raw_value) = line.split_at(1);
        let value: i32 = raw_value.parse().unwrap();

        match command {
            "N" => waypoint.1 += value,
            "E" => waypoint.0 += value,
            "S" => waypoint.1 -= value,
            "W" => waypoint.0 -= value,
            "R" => waypoint = rotate(waypoint, value / 90),
            "L" => waypoint = rotate(waypoint, -1 * value / 90),
            "F" => {
                dx += waypoint.0 * value;
                dy += waypoint.1 * value;
            }
            &_ => {} // default case
        }
    }
    dx.abs() + dy.abs()
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
