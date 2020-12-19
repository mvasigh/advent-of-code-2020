use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn get_input_data() {
    let raw_data = fs::read_to_string("data.txt").expect("Could not read data.txt");
    let mut chunks = raw_data.split("\n\n");
    let raw_rules = chunks.next().expect("Could not read raw rules");
    let raw_messages = chunks.next().expect("Could not read raw messages");

    // Process messages into a usable data structure
    let messages = raw_messages
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();

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
        let value = caps["val"].replace("\"", "");

        if value == "a" || value == "b" {
            replaced_rules.insert(key.to_owned(), value.to_owned());
            replaced_ct += 1;
        }
        rule_nums.insert(key.to_owned(), value.to_owned());
    }

    // Next, replace all numbers with letters using multiple passes
    let rule_len = rule_nums.iter().len();
    let nums_re = Regex::new(r"(\d+)").expect("Could not compile nums_re");
    let all_letters_re = Regex::new(r"^[ab\s|]+$").expect("Could not compile all_letters_re");

    
    while replaced_ct < rule_len {
        // println!("rule_len: {}, replaced_ct: {}", rule_len, replaced_ct);
        'inner: for (key, val) in rule_nums.iter_mut() {
            println!("Key: {}, Val: {}", key, val);
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

    dbg!(&replaced_rules);
}

fn main() {
    get_input_data();
    println!("Hello, world!");
}
