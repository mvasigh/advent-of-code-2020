#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug)]
enum InstructionKind {
    Mask,
    Insert,
}
#[derive(Debug)]
struct Instruction {
    mask: Option<String>,
    address: Option<u64>,
    value: Option<u64>,
    kind: InstructionKind,
}

fn get_input_reader() -> std::io::Result<BufReader<File>> {
    let file = File::open("data.txt")?;
    Ok(BufReader::new(file))
}

fn parse_line(line: String) -> Option<Instruction> {
    lazy_static! {
        static ref MASK_RE: Regex =
            Regex::new(r"mask = (?P<mask>[01X]{36})").expect("Could not compile mask regex");
        static ref INSERT_RE: Regex =
            Regex::new(r"mem\[(?P<mem>\d+)\] = (?P<val>\d+)").expect("Could not compile mask regex");
    };

    if MASK_RE.is_match(&line) {
        let caps = MASK_RE.captures(&line).unwrap();
        Some(Instruction {
            mask: Some(caps["mask"].to_owned()),
            address: None,
            value: None,
            kind: InstructionKind::Mask,
        })
    } else if INSERT_RE.is_match(&line) {
        let caps = INSERT_RE.captures(&line).unwrap();
        Some(Instruction {
            mask: None,
            address: Some(caps["mem"].parse().unwrap()),
            value: Some(caps["val"].parse().unwrap()),
            kind: InstructionKind::Insert
        })
    } else {
        None
    }
}

fn apply_mask(val: u64, mask: &String) -> u64 {
    let val_bin = format!("{:036b}", val);
    let mut new_val = String::with_capacity(36);
    let mask_chars = mask.chars().collect::<Vec<char>>();

    for (i, char) in val_bin.chars().enumerate() {
        let mask_char = mask_chars[i];
        new_val.push(if mask_char == 'X' {
            char
        } else {
            mask_char
        });
    }

    u64::from_str_radix(&new_val, 2).expect("Could not parse binary")
}

fn apply_mask_v2(val: u64, mask: &String) -> Vec<u64> {
    let val_bin = format!("{:036b}", val);
    let mut new_vals: Vec<String> = Vec::new();
    let mask_chars = mask.chars().collect::<Vec<char>>();

    new_vals.push(String::new());

    for (i, char) in val_bin.chars().enumerate() {
        let mask_char = mask_chars[i];
        if mask_char == 'X' {
            // for each string in the vec, make two variants for 0 and 1
            let mut clones: Vec<String> = Vec::new();
            for i in 0..new_vals.len() {
                let string = new_vals.get_mut(i).expect("Could not get mutable string");
                let mut clone = string.to_string();
                
                string.push('0');
                clone.push('1');

                clones.push(clone);
            }
            new_vals.append(&mut clones);
        } else if mask_char == '1' {
            // overwrite each string at this pos with 1
            for i in 0..new_vals.len() {
                let string = new_vals.get_mut(i).expect("Could not get mutable string");
                string.push(mask_char);
            }
        } else {
            // use existing value
            for i in 0..new_vals.len() {
                let string = new_vals.get_mut(i).expect("Could not get mutable string");
                string.push(char);
            }
        }
    }

    new_vals.iter().map(|s| u64::from_str_radix(s, 2).expect("Could not parse binary")).collect::<Vec<u64>>()
}

fn part_one() -> u64 {
    let reader = get_input_reader().expect("Could not open file reader");

    let mut mask = String::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result.expect("Could not get raw line");
        let instr = parse_line(line).expect("Could not parse instruction");

        match instr.kind {
            InstructionKind::Mask => mask = instr.mask.unwrap(),
            InstructionKind::Insert => {
                let new_value = apply_mask(instr.value.unwrap(), &mask);
                let entry = memory.entry(instr.address.unwrap()).or_insert(0);
                *entry = new_value;
            }
        }
    }

    memory.iter().fold(0, |acc, curr| acc + curr.1)
}

fn part_two() -> u64 {
    let reader = get_input_reader().expect("Could not open file reader");

    let mut mask = String::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result.expect("Could not get raw line");
        let instr = parse_line(line).expect("Could not parse instruction");

        match instr.kind {
            InstructionKind::Mask => mask = instr.mask.unwrap(),
            InstructionKind::Insert => {
                let new_addresses = apply_mask_v2(instr.address.unwrap(), &mask);

                for address in new_addresses.iter() {
                    let entry = memory.entry(*address).or_insert(0);
                    *entry = instr.value.unwrap();
                }
            }
        }
    }

    memory.iter().fold(0, |acc, curr| acc + curr.1)
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
