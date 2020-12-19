use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn get_input_data() -> (HashMap<i32, String>, Vec<String>) {
    let raw_data = fs::read_to_string("data.txt").expect("Could not read data.txt");
    let mut chunks = raw_data.split("\n\n");
    let raw_rules = chunks.next().expect("Could not read raw rules");
    let raw_messages = chunks.next().expect("Could not read raw messages");

    // Process messages into a usable data structure
    let messages = raw_messages
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // Process rules into a usable data structure
    // 1. Get rid of all the numbers
    let rule_re = Regex::new(r"(?P<key>\d+): (?P<val>.+)").expect("Could not compile rule regex");
    let mut rule_nums: HashMap<i32, String> = HashMap::new();
    let mut replaced_rules: HashMap<i32, String> = HashMap::new();
    let mut replaced_ct: usize = 0;

    // First, find the root rules that are "a" and "b"
    for raw_rule in raw_rules.split("\n") {
        let caps = rule_re
            .captures(raw_rule)
            .expect("Could not match raw rule");
        let key = caps["key"].parse::<i32>().expect("Could not parse key");
        let mut value = caps["val"].replace("\"", "");

        if value == "a" || value == "b" {
            replaced_rules.insert(key.to_owned(), value.to_owned());
            replaced_ct += 1;
        } else {
            value = value
                .split("|")
                .map(|sec| {
                    sec.split_whitespace()
                        .map(|num| format!("({})", num))
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("|");
        }

        rule_nums.insert(key.to_owned(), value.to_owned());
    }

    // Next, replace all numbers with letters using multiple passes
    let rule_len = rule_nums.iter().len();
    let nums_re = Regex::new(r"(\d+)").expect("Could not compile nums_re");
    let all_letters_re = Regex::new(r"^[ab\s()|]+$").expect("Could not compile all_letters_re");

    // TODO: Make this deterministic
    while replaced_ct < rule_len {
        'inner: for (key, val) in rule_nums.iter_mut() {
            if all_letters_re.is_match(val) {
                continue 'inner;
            }

            // Replace all letter rules with the number version if it exists
            let mut replaced_val = val.to_owned();
            for caps in nums_re.captures_iter(&val.to_string()) {
                let cap = caps
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .expect("Could not parse number");
                if replaced_rules.contains_key(&cap) {
                    let replacement = replaced_rules.get(&cap).unwrap();
                    replaced_val = replaced_val.replace(&cap.to_string(), &replacement);
                }
            }
            *val = replaced_val.to_string();

            // If everything is a number, move into replaced_rules
            if all_letters_re.is_match(&replaced_val) {
                replaced_rules.insert(*key, replaced_val);
                replaced_ct += 1;
            }
        }
    }

    (replaced_rules, messages)
}

fn part_one() -> i32 {
    let (rules, messages) = get_input_data();
    let zero_rule = rules.get(&0).expect("Could not get rule 0");
    let zero_rule_re =
        Regex::new(&format!("^{}$", zero_rule)).expect("Could not compile zero rule regex");

    let count = messages.iter().fold(0, |acc, curr| {
        if zero_rule_re.is_match(curr) {
            acc + 1
        } else {
            acc
        }
    });

    count
}

fn main() {
    println!("Part 1: {}", part_one());
}
