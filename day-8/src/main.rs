use core::panic;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

// !`acc` - increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.

// !`jmp` - jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.

// !`nop` - stands for No OPeration - it does nothing. The instruction immediately below it is executed next.

#[derive(Debug)]
enum Command {
    Jmp,
    Acc,
    Nop,
}
#[derive(Debug)]
struct Instruction {
    cmd: Command,
    val: i32,
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

fn run_til_repeat() -> i32 {
    let instr_vec = get_input_data();
    let mut history: HashSet<i32> = HashSet::new();
    let mut acc = 0;
    let mut pointer: i32 = 0;

    while !history.contains(&pointer) {
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

    acc
}

fn part_one() -> i32 {
    run_til_repeat()
}

fn main() {
    println!("Part 1: {}", part_one());
}
