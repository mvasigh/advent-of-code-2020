use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn get_input_reader() -> io::Result<BufReader<File>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn search(range: Vec<i16>, instructions: &str) -> Option<i16> {
    let mut temp_range = range.clone();
    for instr in instructions.chars() {
        let length = temp_range.len();
        let mut chunks = temp_range.chunks(length / 2);
        let first_half = chunks.next().expect("Could not yield a first half");
        let second_half = chunks.next().expect("Could not yield a second half");

        match instr {
            'F' | 'L' => {
                temp_range = first_half.to_vec();
            }
            'B' | 'R' => {
                temp_range = second_half.to_vec();
            }
            _char => {
                // do nothing
            }
        }

        if temp_range.len() == 1 {
            return Some(temp_range[0]);
        }
    }
    None
}

fn get_seat_id(row: i16, col: i16) -> i16 {
    row * 8 + col
}

fn get_possible_ids() -> Vec<i16> {
    let mut all_ids: Vec<i16> = Vec::new();
    for row in 0..128 {
        for col in 0..8 {
            all_ids.push(get_seat_id(row, col));
        }
    }
    all_ids
}

fn get_all_seat_ids() -> Vec<i16> {
    let mut all_seat_ids: Vec<i16> = Vec::new();
    let reader = get_input_reader().expect("Could not read input file");

    for line in reader.lines() {
        let instructions = line.expect("Could not read line");

        let row = search((0..128).collect::<Vec<i16>>(), &instructions[..7])
            .expect("Could not calculate row");
        let col = search((0..8).collect::<Vec<i16>>(), &instructions[7..])
            .expect("Could not calculate column");

        let id = get_seat_id(row, col);
        all_seat_ids.push(id);
    }
    all_seat_ids
}

fn part_one() -> i16 {
    let all_seat_ids = get_all_seat_ids();
    let mut max = 0;

    for seat_id in all_seat_ids.iter() {
        if seat_id > &max {
            max = seat_id.clone()
        }
    }
    max
}

fn part_two() -> i16 {
    let possible_ids = get_possible_ids();
    let all_ids = get_all_seat_ids();
    let max = all_ids.iter().max().unwrap();
    let min = all_ids.iter().min().unwrap();

    let exclusion = possible_ids.iter().filter(|id| !all_ids.contains(id));

    for id in exclusion {
        if id < max && id > min {
            return id.clone();
        }
    }
    -1
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
