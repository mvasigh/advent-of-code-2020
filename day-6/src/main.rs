use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

struct InputDataReader {
    lines: Lines<BufReader<File>>,
}

impl InputDataReader {
    fn new(file_name: &str) -> Result<InputDataReader, std::io::Error> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        Ok(InputDataReader { lines: lines })
    }
}

impl Iterator for InputDataReader {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut owned_str = String::new();
        while let Some(Ok(line)) = self.lines.next() {
            if line.is_empty() {
                return Some(owned_str);
            }
            owned_str += &line;
        }
        if owned_str.is_empty() {
            return None;
        }
        Some(owned_str)
    }
}

fn get_input_data() -> InputDataReader {
    InputDataReader::new("data.txt").expect(
        "Something went wrong while reading input data
",
    )
}

fn part_one() -> usize {
    let input_reader = get_input_data();
    let mut count = 0;

    for group in input_reader {
        let chars = group.chars().collect::<std::collections::HashSet<char>>();
        count += chars.len();
    }

    count
}

fn main() {
    println!("Part 1: {}", part_one());
}
