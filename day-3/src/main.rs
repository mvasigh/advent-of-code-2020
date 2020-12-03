use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn get_input_reader() -> io::Result<BufReader<File>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn traverse_slope(right: usize, down: usize) -> i32 {
    let reader = get_input_reader().unwrap();

    let mut tree_count = 0;
    let mut x: usize = 0;

    for (i, line) in reader.lines().enumerate() {
        if i % down > 0 {
            continue;
        }

        let raw = line.unwrap();
        let mut chars = raw.chars();
        let length = raw.len();
        let current = chars.nth(x % length).unwrap();

        if current == '#' {
            tree_count += 1;
        }

        x += right;
    }

    tree_count
}

fn part_one() -> i32 {
    traverse_slope(3, 1)
}

fn part_two() -> i32 {
    traverse_slope(1, 1)
        * traverse_slope(3, 1)
        * traverse_slope(5, 1)
        * traverse_slope(7, 1)
        * traverse_slope(1, 2)
}

fn main() {
    let part_one_answer = part_one();
    println!("Part 1: {}", part_one_answer);

    let part_two_answer = part_two();
    println!("Part 2: {}", part_two_answer);
}
