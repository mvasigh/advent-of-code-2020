use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn get_input_data() -> (HashMap<String, HashSet<i32>>, Vec<i32>, Vec<Vec<i32>>) {
    let raw_data = fs::read_to_string("data.txt").expect("Could not read input data");
    let mut chunks = raw_data.split("\n\n");
    let raw_ranges = chunks.next().expect("Could not get ranges");
    let raw_your_ticket = chunks.next().expect("Could not get your ticket");
    let raw_other_tickets = chunks.next().expect("Could not get other tickets");

    // Process ranges
    let field_re = Regex::new(r"(?P<fieldname>[\w\s]+): (?P<r1start>\d+)-(?P<r1end>\d+) or (?P<r2start>\d+)-(?P<r2end>\d+)").expect("Failed to compile field regex");
    let mut fields: HashMap<String, HashSet<i32>> = HashMap::new();

    for line in raw_ranges.split("\n") {
        let caps = field_re
            .captures(line)
            .expect("Could not match 'fields' line");
        let mut all_possible_vals: HashSet<i32> = HashSet::new();
        let r1_start = caps["r1start"].parse::<i32>().unwrap();
        let r1_end = caps["r1end"].parse::<i32>().unwrap();
        let r2_start = caps["r2start"].parse::<i32>().unwrap();
        let r2_end = caps["r2end"].parse::<i32>().unwrap();

        for i in r1_start..r2_end + 1 {
            if (i >= r1_start && i <= r1_end) || (i >= r2_start && i <= r2_end) {
                all_possible_vals.insert(i);
            }
        }
        fields.insert(caps["fieldname"].to_owned(), all_possible_vals);
    }

    let ticket_re = Regex::new(r"(\d+)").expect("Could not compile ticket regex");
    // Process your ticket
    let your_ticket = ticket_re
        .captures_iter(raw_your_ticket)
        .map(|c| c[0].parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // Process other ticket
    let mut other_tickets: Vec<Vec<i32>> = Vec::new();
    for line in raw_other_tickets.split("\n").skip(1) {
        other_tickets.push(
            ticket_re
                .captures_iter(line)
                .map(|c| c[0].parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }

    (fields, your_ticket, other_tickets)
}

fn part_one() -> i32 {
    let mut invalid_sum = 0;
    let (mut fields, _, other_tickets) = get_input_data();

    let all_valid_values: HashSet<i32> =
        fields
            .iter_mut()
            .fold(HashSet::new(), |mut acc, (_, set)| {
                acc.extend(set.iter().copied().collect::<HashSet<i32>>());
                acc
            });

    for ticket in other_tickets.iter() {
        for value in ticket.iter() {
            if !all_valid_values.contains(value) {
                invalid_sum += value;
            }
        }
    }

    invalid_sum
}

fn main() {
    println!("Part 1: {}", part_one());
}
