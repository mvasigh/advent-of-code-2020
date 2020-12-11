use std::fs;
use std::io;

#[derive(Clone, Debug, std::cmp::PartialEq)]
enum CellState {
    Full,
    Empty,
    Floor,
}

enum CountingStyle {
    Adjacent,
    LineOfSight,
}

enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

static DIRECTIONS: &'static [Direction] = &[
    Direction::North,
    Direction::NorthWest,
    Direction::West,
    Direction::SouthWest,
    Direction::South,
    Direction::SouthEast,
    Direction::East,
    Direction::NorthEast,
];

fn read_input_data() -> Result<Vec<Vec<CellState>>, io::Error> {
    let raw_data = fs::read_to_string("data.txt")?;
    Ok(raw_data
        .trim()
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

fn count_adjacent_neighbors(game_state: &Vec<Vec<CellState>>, r: usize, c: usize) -> u8 {
    let mut filled_neighbors: u8 = 0;
    let row_len = game_state[0].len();
    let col_len = game_state.len();

    // Check col to the left
    if c != 0 {
        if r != 0 && game_state[r - 1][c - 1] == CellState::Full {
            filled_neighbors += 1;
        }
        if game_state[r][c - 1] == CellState::Full {
            filled_neighbors += 1;
        }
        if r != col_len - 1 && game_state[r + 1][c - 1] == CellState::Full {
            filled_neighbors += 1;
        }
    }

    // Check col to the right
    if c != row_len - 1 {
        if r != 0 && game_state[r - 1][c + 1] == CellState::Full {
            filled_neighbors += 1;
        }
        if game_state[r][c + 1] == CellState::Full {
            filled_neighbors += 1;
        }
        if r != col_len - 1 && game_state[r + 1][c + 1] == CellState::Full {
            filled_neighbors += 1;
        }
    }

    // Check directly above
    if r != 0 && game_state[r - 1][c] == CellState::Full {
        filled_neighbors += 1;
    }

    // Check directly below
    if r != col_len - 1 && game_state[r + 1][c] == CellState::Full {
        filled_neighbors += 1;
    }

    filled_neighbors
}

fn count_line_of_sight_neighbors(game_state: &Vec<Vec<CellState>>, r: usize, c: usize) -> u8 {
    let mut filled_neighbors: u8 = 0;
    let row_len = game_state.len();
    let col_len = game_state[0].len();

    'outer: for dir in DIRECTIONS.iter() {
        let (dr, dc): (i8, i8) = match dir {
            &Direction::North => (-1, 0),
            &Direction::NorthEast => (-1, 1),
            &Direction::East => (0, 1),
            &Direction::SouthEast => (1, 1),
            &Direction::South => (1, 0),
            &Direction::SouthWest => (1, -1),
            &Direction::West => (0, -1),
            &Direction::NorthWest => (-1, -1),
        };

        let mut pr = (r as i8) + dr;
        let mut pc = (c as i8) + dc;

        while 
            pc < (col_len as i8) 
            && pc >= 0 
            && pr < (row_len as i8)
            && pr >= 0 
        {
            let cell = &game_state[pr as usize][pc as usize];
            if game_state[pr as usize][pc as usize] != CellState::Floor {
                // println!("It's full!");
                filled_neighbors += match cell {
                    CellState::Full => 1,
                    CellState::Empty => 0,
                    _ => 0,
                };
                continue 'outer;
            }
            pr += dr;
            pc += dc;
        }
    }

    filled_neighbors
}

fn run_game(
    prev_state: &Vec<Vec<CellState>>,
    counting_style: &CountingStyle,
) -> (Vec<Vec<CellState>>, u16) {
    let mut mutations: u16 = 0;
    let mut new_state = prev_state.to_vec();

    for (r, row) in prev_state.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            let (neighbors, tolerated) = match counting_style {
                CountingStyle::Adjacent => (count_adjacent_neighbors(prev_state, r, c), 4),
                CountingStyle::LineOfSight => (count_line_of_sight_neighbors(prev_state, r, c), 5)
            };

            // Mutate new state if neighbors exceed 3
            if col == &CellState::Full && neighbors >= tolerated {
                new_state[r][c] = CellState::Empty;
                mutations += 1;
            } else if col == &CellState::Empty && neighbors == 0 {
                new_state[r][c] = CellState::Full;
                mutations += 1;
            }
        }
    }

    (new_state, mutations)
}

fn get_full_seats(counting_style: CountingStyle) -> i32 {
    let mut game_state = read_input_data().expect("Could not read initial input");
    let mut mutations = 0;
    let mut iterations = 0;

    while mutations != 0 || iterations == 0 {
        let (new_complete_state, new_mutations) = run_game(&game_state, &counting_style);

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

fn part_one() -> i32 {
    get_full_seats(CountingStyle::Adjacent)
}

fn part_two() -> i32 {
    get_full_seats(CountingStyle::LineOfSight)
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
