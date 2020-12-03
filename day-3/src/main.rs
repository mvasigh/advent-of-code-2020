use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn get_input_reader() -> io::Result<BufReader<File>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn part_one() -> i32 {
    let reader = get_input_reader().unwrap();

    let mut tree_count = 0;
    let mut x: usize = 0;

    for line in reader.lines() {
        let raw = line.unwrap();
        let mut chars = raw.chars();
        let length = raw.len();
        let current = chars.nth(x % length).unwrap();

        if current == '#' {
            tree_count += 1;
        }

        x += 3;
    }

    tree_count
}

fn main() {
    let part_one_answer = part_one();
    println!("Part 1: {}", part_one_answer);
}
