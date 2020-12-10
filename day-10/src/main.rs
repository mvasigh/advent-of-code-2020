use std::fs;
use std::io;
use std::collections::HashMap;

fn get_input_data() -> Result<Vec<u16>, io::Error> {
    let raw_data = fs::read_to_string("data.txt")?;
    Ok(raw_data
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u16>>())
}

fn part_one() -> u16 {
    let mut input_adapters = get_input_data().expect("Failed to read input data");
    let mut adapters: Vec<u16> = Vec::new();
    // Account for charging port
    adapters.push(0);
    adapters.append(&mut input_adapters);
    adapters.sort();
    // Account for device
    adapters.push(adapters[adapters.len() - 1] + 3);
    
    // Calculate a map of all of the deltas
    let deltas: HashMap<u16, u16> = adapters
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, curr)| {
            if i != 0 {
                let diff = curr - adapters[i - 1];
                let entry = acc.entry(diff).or_insert(0);
                *entry += 1;
            }
            acc
        });

    // Figure out deltas of 1 multiplied by deltas of 3
    deltas.get(&1).unwrap() * deltas.get(&3).unwrap()
}

fn main() {
    println!("Part 1: {}", part_one());
}
