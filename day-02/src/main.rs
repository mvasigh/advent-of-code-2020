use std::fs;
#[derive(Debug)]
struct Password {
    range: (i32, i32),
    special_char: char,
    input: String,
}

impl Password {
    fn is_valid_part_one(&self) -> bool {
        let count = self.input.matches(self.special_char).count() as i32;
        let (min, max) = self.range;
        count >= min && count <= max
    }

    fn is_valid_part_two(&self) -> bool {
        let first_pos = (self.range.0 - 1) as usize;
        let second_pos = (self.range.1 - 1) as usize;

        let chars: Vec<char> = self.input.chars().collect();

        if chars[first_pos] == self.special_char && chars[second_pos] == self.special_char {
            return false;
        } else {
            return chars[first_pos] == self.special_char || chars[second_pos] == self.special_char;
        }
    }
}

fn get_input_data() -> Vec<Password> {
    let raw_data = fs::read_to_string("data.txt").unwrap();
    let passwords: Vec<Password> = raw_data
        .lines()
        .map(|val| val.trim())
        .map(|raw| -> Password {
            let processed = raw.replace(':', "");
            let mut components = processed.split(" ");

            let range: Vec<i32> = components
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse().unwrap())
                .collect();
            let special_char = components.next().unwrap().chars().next().unwrap();
            let input = components.next().unwrap();

            Password {
                range: (range[0], range[1]),
                special_char: special_char,
                input: input.to_string(),
            }
        })
        .collect();

    return passwords;
}

fn part_one() -> i32 {
    let passwords = get_input_data();
    let mut num_valid = 0;

    for password in passwords.iter() {
        if password.is_valid_part_one() {
            num_valid += 1;
        }
    }

    num_valid
}

fn part_two() -> i32 {
    let passwords = get_input_data();
    let mut num_valid = 0;

    for password in passwords.iter() {
        if password.is_valid_part_two() {
            num_valid += 1;
        }
    }

    num_valid
}

fn main() {
    let part_one_answer = part_one();
    println!("Part 1: {}", part_one_answer);

    let part_two_answer = part_two();
    println!("Part 2: {}", part_two_answer);
}
