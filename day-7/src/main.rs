use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn get_input_reader() -> io::Result<BufReader<File>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn get_container_input() -> HashMap<String, HashMap<String, u16>> {
    let container_re =
        Regex::new(r"([\w ]+) bags contain").expect("Could not compile container regex");
    let contents_re = Regex::new(r"(?P<num>\d) (?P<bagtype>[\w\s]+) bag")
        .expect("Could not compile contents regex");

    let mut all_bags: HashMap<String, HashMap<String, u16>> = HashMap::new();

    for line in get_input_reader().expect("Could not read file").lines() {
        let line_str = line.expect("Could not read line");
        if !contents_re.is_match(&line_str) {
            continue;
        }

        let container = container_re
            .captures(&line_str)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        for caps in contents_re.captures_iter(&line_str) {
            let record = all_bags
                .entry(caps["bagtype"].to_string())
                .or_insert(HashMap::new());

            record.insert(container.to_string(), caps["num"].parse().unwrap());
        }
    }

    all_bags
}

fn find_containers(input: &HashMap<String, HashMap<String, u16>>, key: &str) -> HashSet<String> {
    if !input.contains_key(key) {
        return HashSet::new();
    }

    let contained_by_map = input.get(key).expect("Could not find the correct key");
    let mut containers = contained_by_map
        .keys()
        .map(|s| s.to_owned())
        .collect::<HashSet<String>>();

    if containers.len() > 0 {
        for container_key in contained_by_map.keys() {
            let container_containers = find_containers(input, container_key);
            containers = containers
                .union(&container_containers)
                .map(|s| s.to_owned())
                .collect::<HashSet<String>>()
        }
    }

    containers
}

fn part_one() -> usize {
    let containers = find_containers(&get_container_input(), "shiny gold");
    containers.len()
}

// fn part_two() -> usize {

// }

fn main() {
    println!("Part 1: {}", part_one());
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_test() {}
}
