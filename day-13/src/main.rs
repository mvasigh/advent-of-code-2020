use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input_data() -> (i32, Vec<i32>) {
    let file = File::open("data.txt").expect("Could not open input file");
    let mut iter = BufReader::new(file).lines();
    let timestamp: i32 = iter
        .next()
        .unwrap()
        .expect("Could not read timestamp")
        .parse()
        .unwrap();
    let ids = iter
        .next()
        .unwrap()
        .expect("Could not read ID list")
        .split(",")
        .filter(|s| *s != "x")
        .map(|s| s.parse().expect("Could not parse ID"))
        .collect::<Vec<i32>>();

    (timestamp, ids)
}

fn part_one() -> i32 {
    let (timestamp, ids) = read_input_data();

    let mut best_bus = 0;
    let mut min_wait = 0;

    for (i, id) in ids.iter().enumerate() {
        let wait = id - (timestamp % id);
        if i == 0 || wait < min_wait {
            best_bus = *id;
            min_wait = wait;
        }
    }

    best_bus * min_wait
}

fn main() {
    println!("Part 1: {}", part_one());
}
