use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn get_input_data() -> (HashMap<String, HashSet<i32>>, Vec<i32>, Vec<Vec<i32>>) {
    let raw_data = fs::read_to_string("data.txt").expect("Could not read input data");
    let mut chunks = raw_data.split("\n\n");
    let raw_ranges = chunks.next().expect("Could not get ranges");
    let raw_your_ticket = chunks.next().expect("Could not get your ticket");
    let raw_other_tickets = chunks.next().expect("Could not get other tickets");

    let field_re = Regex::new(r"(?P<fieldname>[\w\s]+): (?P<r1start>\d+)-(?P<r1end>\d+) or (?P<r2start>\d+)-(?P<r2end>\d+)").expect("Failed to compile field regex");
    let mut fields: HashMap<String, HashSet<i32>> = HashMap::new();

    for line in raw_ranges.split("\n") {
        let caps = field_re
            .captures(line)
            .expect("Could not match 'fields' line");
        let mut set = (caps["r1start"].parse::<i32>().unwrap()
            ..caps["r1end"].parse::<i32>().unwrap() + 1)
            .collect::<HashSet<i32>>();
        set.extend(
            caps["r2start"].parse::<i32>().unwrap()..caps["r2end"].parse::<i32>().unwrap() + 1,
        );
        fields.insert(caps["fieldname"].to_owned(), set);
    }

    let ticket_re = Regex::new(r"(\d+)").expect("Could not compile ticket regex");
    let your_ticket = ticket_re
        .captures_iter(raw_your_ticket)
        .map(|c| c[0].parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

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

fn get_all_valid_tickets(
    fields: &HashMap<String, HashSet<i32>>,
    tickets: &Vec<Vec<i32>>,
) -> (Vec<Vec<i32>>, i32) {
    let mut valid_tickets: Vec<Vec<i32>> = Vec::new();
    let mut invalid_sum = 0;

    let all_valid_values: HashSet<i32> =
        fields
            .to_owned()
            .iter_mut()
            .fold(HashSet::new(), |mut acc, (_, set)| {
                acc.extend(set.iter().copied().collect::<HashSet<i32>>());
                acc
            });

    'outer: for ticket in tickets.iter() {
        for value in ticket.iter() {
            if !all_valid_values.contains(value) {
                invalid_sum += value;
                continue 'outer;
            }
        }
        valid_tickets.push(ticket.to_owned());
    }

    (valid_tickets, invalid_sum)
}

fn exclude_multi_candidates(candidates: &Vec<HashSet<&String>>) -> HashSet<String> {
    candidates
        .to_vec()
        .iter()
        .filter(|c| c.len() == 1)
        .map(|c| c.iter().next().unwrap().to_string())
        .collect::<HashSet<String>>()
}

fn part_one() -> i32 {
    let (fields, _, other_tickets) = get_input_data();

    let (_, invalid_sum) = get_all_valid_tickets(&fields, &other_tickets);

    invalid_sum
}

fn part_two() -> u64 {
    let (fields, your_ticket, other_tickets) = get_input_data();

    let (valid_tickets, _) = get_all_valid_tickets(&fields, &other_tickets);

    let len = valid_tickets[0].len();
    let mut field_candidates: Vec<HashSet<&String>> = vec![HashSet::new(); len];

    for i in 0..len {
        let mut candidates = fields.keys().collect::<HashSet<&String>>();

        for ticket in valid_tickets.iter() {
            for field in candidates.to_owned().iter() {
                let possible_values = fields
                    .get(field.to_owned())
                    .expect("Could not find set of values");
                let value = &ticket[i];

                if !possible_values.contains(value) {
                    candidates.remove(field);
                }
            }
        }
        if candidates.len() >= 1 {
            field_candidates[i] = candidates;
        }
    }

    let mut finalized = exclude_multi_candidates(&field_candidates);

    while finalized.len() < len {
        for field in field_candidates.iter_mut() {
            if field.len() > 1 {
                for final_field in finalized.iter() {
                    field.remove(final_field);
                }
            }
        }

        finalized = exclude_multi_candidates(&field_candidates);
    }

    field_candidates
        .iter()
        .enumerate()
        .map(|(i, f)| (i, f.iter().next().unwrap().to_string()))
        .fold(1, |mut acc, (i, f)| {
            if f.starts_with("departure") {
                acc *= your_ticket[i] as u64
            }
            acc
        })
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
