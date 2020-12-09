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

fn main() {
    println!("Part 1: {}", part_one());
}
