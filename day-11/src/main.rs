use std::fs;
use std::io;

#[derive(Clone, Debug, std::cmp::PartialEq)]
enum CellState {
    Full,
    Empty,
    Floor,
}

fn read_input_data() -> Result<Vec<Vec<CellState>>, io::Error> {
    let raw_data = fs::read_to_string("data.txt")?;
    Ok(raw_data
        .split_whitespace()
        .map(|r| {
            r.chars()
                .map(|c| match c {
                    'L' => CellState::Empty,
                    '#' => CellState::Full,
                    '.' => CellState::Floor,
                    _ => panic!("Unexpected char found when parsing inp"),
                })
                .collect::<Vec<CellState>>()
        })
        .collect::<Vec<Vec<CellState>>>())
}

fn run_game(prev_state: &Vec<Vec<CellState>>) -> (Vec<Vec<CellState>>, u16) {
    let mut mutations: u16 = 0;
    let mut new_state = prev_state.to_vec();

    let col_len = prev_state.len();
    for (r, row) in prev_state.iter().enumerate() {
        let row_len = row.len();
        for (c, col) in row.iter().enumerate() {
            let mut filled_neighbors: u8 = 0;

            // Check col to the left
            if c != 0 {
                if r != 0 && prev_state[r - 1][c - 1] == CellState::Full {
                    filled_neighbors += 1;
                }
                if prev_state[r][c - 1] == CellState::Full {
                    filled_neighbors += 1;
                }
                if r != col_len - 1 && prev_state[r + 1][c - 1] == CellState::Full {
                    filled_neighbors += 1;
                }
            }

            // Check col to the right
            if c != row_len - 1 {
                if r != 0 && prev_state[r - 1][c + 1] == CellState::Full {
                    filled_neighbors += 1;
                }
                if prev_state[r][c + 1] == CellState::Full {
                    filled_neighbors += 1;
                }
                if r != col_len - 1 && prev_state[r + 1][c + 1] == CellState::Full {
                    filled_neighbors += 1;
                }
            }

            // Check directly above
            if r != 0 && prev_state[r - 1][c] == CellState::Full {
                filled_neighbors += 1;
            }

            // Check directly below
            if r != col_len - 1 && prev_state[r + 1][c] == CellState::Full {
                filled_neighbors += 1;
            }

            // Mutate new state if neighbors exceed 3
            if col == &CellState::Full && filled_neighbors >= 4 {
                new_state[r][c] = CellState::Empty;
                mutations += 1;
            } else if col == &CellState::Empty && filled_neighbors == 0 {
                new_state[r][c] = CellState::Full;
                mutations += 1;
            }
        }
    }

    (new_state, mutations)
}

fn part_one() -> i32 {
    let mut game_state = read_input_data().expect("Could not read initial input");
    let mut mutations = 0;
    let mut iterations = 0;

    while mutations != 0 || iterations == 0 {
        let (new_complete_state, new_mutations) = run_game(&game_state);

        mutations = new_mutations;
        game_state = new_complete_state.to_vec();

        if mutations == 0 {
            break;
        }

        iterations += 1;
    }

    // Count how many seats are occupied in game state
    game_state.into_iter().flatten().fold(0, |acc, c| match c {
        CellState::Full => acc + 1,
        _ => acc,
    })
}

fn main() {
    println!("Part 1: {}", part_one());
}
