use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input_data() -> (i32, Vec<Option<i32>>) {
    let file = File::open("data.txt").expect("Could not open input file");
    let mut iter = BufReader::new(file).lines();
    let timestamp: i32 = iter
        .next()
        .unwrap()
        .expect("Could not read timestamp")
        .parse()
        .unwrap();
    let buses = iter
        .next()
        .unwrap()
        .expect("Could not read ID list")
        .split(",")
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse::<i32>().unwrap())
            }
        })
        .collect();

    (timestamp, buses)
}

fn part_one() -> i32 {
    let (timestamp, buses) = read_input_data();
    let ids = buses
        .iter()
        .filter(|b| b.is_some())
        .map(|id| id.unwrap())
        .collect::<Vec<i32>>();

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

fn part_two() -> u64 {
    let (_, buses) = read_input_data();

    let eligible = buses
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .map(|b| (b.0, b.1.unwrap()))
        .collect::<Vec<(usize, i32)>>();

    let mut max: u64 = 1;
    let raw_t = eligible
        .iter()
        .map(|(i, id)| {
            let mut a = id - *i as i32;
            while a < 0 {
                a += *id;
            }
            let m: u64 = eligible
                .to_vec()
                .iter()
                .filter(|c| c.0 != *i)
                .fold(1, |acc: u64, (_, val)| acc * *val as u64);

            let coeff = m % *id as u64;
            let mut y = 1;
            if coeff > 0 {
                max *= *id as u64;
                while (coeff * y) % *id as u64 != 1 {
                    y += 1;
                }
            }
            (m, y, a)
        })
        .fold(0, |acc, curr| acc + (curr.0 * curr.1 * curr.2 as u64));

    raw_t % max
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
