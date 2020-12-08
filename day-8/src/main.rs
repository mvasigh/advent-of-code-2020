use core::panic;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
enum Command {
    Jmp,
    Acc,
    Nop,
}
#[derive(Debug, Clone)]
struct Instruction {
    cmd: Command,
    val: i32,
}

enum ExitCondition {
    Loop,
    Overflow,
    Valid,
}

fn get_input_data() -> Vec<Instruction> {
    let instruction_re =
        Regex::new(r"(?P<cmd>[a-z]{3}) (?P<val>[\+|-]\d+)").expect("Could not compile regex");
    let raw_data = fs::read_to_string("data.txt").expect("Could not read data file");

    let vec = raw_data
        .split('\n')
        .map(|s| {
            let caps = instruction_re
                .captures(s)
                .expect("Could not match instruction");
            let cmd = match &caps["cmd"] {
                "jmp" => Command::Jmp,
                "acc" => Command::Acc,
                "nop" => Command::Nop,
                &_ => panic!("Unknown command detected!"),
            };
            let val: i32 = caps["val"].to_owned().parse().expect("Could not parse");

            Instruction { cmd: cmd, val: val }
        })
        .collect::<Vec<Instruction>>();

    return vec;
}

fn run_till_uncorrupt() -> i32 {
    let instr_vec = get_input_data();

    let mut acc = 0;

    for (i, instr) in instr_vec.iter().enumerate() {
        match instr.cmd {
            Command::Jmp => {
                let mut cloned_vec = instr_vec.to_vec();
                cloned_vec[i] = Instruction {
                    cmd: Command::Nop,
                    val: instr.val,
                };
                let (cloned_acc, exit_condition) = run_program(&cloned_vec);
                match exit_condition {
                    ExitCondition::Valid => {
                        acc = cloned_acc;
                    }
                    _ => {}
                }
            }
            Command::Nop => {
                let mut cloned_vec = instr_vec.to_vec();
                cloned_vec[i] = Instruction {
                    cmd: Command::Jmp,
                    val: instr.val,
                };
                let (cloned_acc, exit_condition) = run_program(&cloned_vec);
                match exit_condition {
                    ExitCondition::Valid => {
                        acc = cloned_acc;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    acc
}

fn run_program(instr_vec: &Vec<Instruction>) -> (i32, ExitCondition) {
    let mut history: HashSet<i32> = HashSet::new();
    let instr_len = instr_vec.len();
    let mut acc = 0;
    let mut pointer: i32 = 0;

    while !history.contains(&pointer) && pointer < instr_len as i32 {
        history.insert(pointer);
        let instr = &instr_vec[pointer as usize];
        match instr.cmd {
            Command::Acc => {
                acc += instr.val;
                pointer += 1;
            }
            Command::Jmp => {
                pointer += instr.val;
            }
            Command::Nop => {
                pointer += 1;
            }
        }
    }

    if history.contains(&pointer) {
        (acc, ExitCondition::Loop)
    } else if pointer == instr_len as i32 {
        (acc, ExitCondition::Valid)
    } else {
        (acc, ExitCondition::Overflow)
    }
}

fn part_one() -> i32 {
    let input_data = get_input_data();
    let result = run_program(&input_data);
    result.0
}

fn part_two() -> i32 {
    run_till_uncorrupt()
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
