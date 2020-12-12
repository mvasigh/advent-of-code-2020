use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

struct ResponseGroup {
    responses: String,
    size: u16,
}

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
    type Item = ResponseGroup;

    fn next(&mut self) -> Option<ResponseGroup> {
        let mut responses = String::new();
        let mut size: u16 = 0;

        while let Some(Ok(line)) = self.lines.next() {
            if line.is_empty() {
                return Some(ResponseGroup {
                    responses: responses,
                    size: size,
                });
            }
            size += 1;
            responses += &line;
        }
        if responses.is_empty() {
            return None;
        }
        Some(ResponseGroup {
            responses: responses,
            size: size,
        })
    }
}

fn get_input_data() -> InputDataReader {
    InputDataReader::new("data.txt").expect("Something went wrong while reading input data")
}

fn part_one() -> usize {
    let input_reader = get_input_data();
    let mut count = 0;

    for group in input_reader {
        let chars = group.responses.chars().collect::<HashSet<char>>();
        count += chars.len();
    }
    count
}

fn part_two() -> i32 {
    let input_reader = get_input_data();
    let mut count = 0;

    for group in input_reader {
        let responses = group.responses;
        let mut charmap: HashMap<char, u16> = HashMap::new();

        for character in responses.chars() {
            let counter = charmap.entry(character).or_insert(0);
            *counter += 1;
        }

        for val in charmap.values() {
            if val == &group.size {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
