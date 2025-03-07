#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;

fn get_input_data() -> Vec<String> {
    let raw_data = fs::read_to_string("data.txt").expect("Failed to read file");
    let raw_passports = raw_data
        .split("\n\n")
        .map(|s| s.replace("\n", " "))
        .collect();

    raw_passports
}

fn is_valid(candidate: &String, exhaustive: bool) -> bool {
    lazy_static! {
        static ref PARTS_RE: Regex =
            Regex::new(r"([a-z]{3}):([^\s]+)").expect("Failed to compile parts regex");
        static ref HCL_RE: Regex =
            Regex::new("^#[a-f0-9]{6}$").expect("Failed to compile hcl regex");
        static ref ECL_RE: Regex =
            Regex::new("^amb|blu|brn|gry|grn|hzl|oth$").expect("Failed to compile ecl regex");
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").expect("Failed to compile pid regex");
        static ref HGT_RE: Regex =
            Regex::new(r"^(\d+)(cm|in)$").expect("Failed to compile hgt regex");
    }

    let mut count = 0;

    for caps in PARTS_RE.captures_iter(candidate) {
        count += 1;

        if exhaustive {
            let key = &caps[1];
            let value = &caps[2];

            let valid_key_val: bool = match key {
                "hgt" => {
                    if !HGT_RE.is_match(value) {
                        return false;
                    }
                    let hgt_caps = HGT_RE.captures(value).expect("Could not get hgt captures");
                    let unit = hgt_caps
                        .get(2)
                        .expect("Could not get capture group 3")
                        .as_str();
                    let measurement: u16 = hgt_caps.get(1).unwrap().as_str().parse().unwrap();

                    match unit {
                        "cm" => measurement >= 150 && measurement <= 193,
                        "in" => measurement >= 59 && measurement <= 76,
                        &_ => false,
                    }
                }
                "byr" => {
                    let byr: u16 = value.parse().expect("Could not parse byr");
                    byr >= 1920 && byr <= 2002
                }
                "iyr" => {
                    let iyr: u16 = value.parse().expect("Could not parse iyr");
                    iyr >= 2010 && iyr <= 2020
                }
                "eyr" => {
                    let eyr: u16 = value.parse().expect("Could not parse eyr");
                    eyr >= 2020 && eyr <= 2030
                }
                "hcl" => HCL_RE.is_match(value),
                "ecl" => ECL_RE.is_match(value),
                "pid" => PID_RE.is_match(value),
                &_ => key == "cid",
            };

            if !valid_key_val {
                return false;
            }
        }
    }

    if count == 8 || (count == 7 && !candidate.contains("cid:")) {
        return true;
    }

    false
}

fn part_one() -> usize {
    get_input_data()
        .iter()
        .filter(|p| is_valid(p, false))
        .count()
}

fn part_two() -> usize {
    get_input_data()
        .iter()
        .filter(|p| is_valid(p, true))
        .count()
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
