use std::io;

fn get_input_data() -> io::Result<Vec<u64>> {
    let raw_input = std::fs::read_to_string("data.txt")?;
    Ok(raw_input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>())
}

fn find_sum_pair(candidates: Vec<u64>, target: &u64) -> Option<u64> {
    for (i, val1) in candidates.iter().enumerate() {
        let pairs = &candidates[(i + 1)..candidates.len()].to_vec();
        for val2 in pairs.iter() {
            if val1 + val2 == *target {
                return Some(*target);
            }
        }
    }
    None
}

fn find_sum_contiguous(candidates: Vec<u64>, target: u64) -> Option<Vec<u64>> {
    'outer: for (i, val1) in candidates.iter().enumerate() {
        if val1 == &target {
            return None;
        }

        let mut set: Vec<u64> = Vec::new();
        let mut sum = *val1;
        let slice = &candidates[(i + 1)..candidates.len()].to_vec();

        for val2 in slice.iter() {
            sum += val2;
            set.push(*val2);
            if sum > target {
                continue 'outer;
            } else if sum == target {
                return Some(set);
            }
        }
    }
    None
}

fn part_one() -> u64 {
    let input_data = get_input_data().expect("Could not retrieve input data");
    let preamble_ct = 25;

    for (i, value) in input_data.iter().enumerate() {
        if i < preamble_ct {
            continue;
        }
        let slice = &input_data[(i - preamble_ct)..i];
        let pair = find_sum_pair(slice.to_vec(), value);
        if pair.is_none() {
            return *value;
        }
    }
    0
}

fn part_two() -> u64 {
    let part_one_answer = part_one();
    let input_data = get_input_data().expect("Could not retrieve input data");
    let mut contiguous_set =
        find_sum_contiguous(input_data, part_one_answer).expect("No contiguous set found");

    contiguous_set.sort();

    contiguous_set[0] + contiguous_set[contiguous_set.len() - 1]
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
