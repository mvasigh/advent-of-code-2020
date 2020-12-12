use std::fs;

fn get_input_data() -> Vec<i32> {
    let mut data: Vec<i32> = fs::read_to_string("data.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    data.sort();

    return data;
}

fn part_1() -> Option<i32> {
    let data = get_input_data();

    for first in data.iter() {
        for second in data.iter() {
            let sum: i32 = first + second;
            if sum == 2020 {
                return Some(first * second);
            }
        }
    }

    None
}

fn part_2() -> Option<i32> {
    let data = get_input_data();

    for first in data.iter() {
        for second in data.iter() {
            for third in data.iter() {
                let sum = first + second + third;
                if sum == 2020 {
                    return Some(first * second * third);
                }
            }
        }
    }

    None
}

fn main() {
    let part_one_answer = part_1().unwrap();
    println!("Part 1 answer: {}", part_one_answer);

    let part_two_answer = part_2().unwrap();
    println!("Part 2 answer: {}", part_two_answer);
}
