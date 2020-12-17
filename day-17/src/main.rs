use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct CellCoords {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Cell {
    coords: CellCoords,
    active: bool,
}

impl Cell {
    fn activate(&mut self) {
        self.active = true;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

#[derive(PartialEq, Debug)]
enum Dimensions {
    Three,
    Four
}

fn get_input_data() -> HashMap<(i32, i32, i32, i32), Cell> {
    let input_raw = r".##.##..
                           ..###.##
                           .##....#
                           ###..##.
                           #.###.##
                           .#.#..#.
                           .......#
                           .#..#..#";

    let mut cell_map: HashMap<(i32, i32, i32, i32), Cell> = HashMap::new();
    for (y, line) in input_raw.trim().split("\n").enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            let active = char == '#';
            let coords = CellCoords {
                x: x as i32,
                y: y as i32,
                z: 1,
                w: 1,
            };

            cell_map.insert(
                (x as i32, y as i32, 1, 1),
                Cell {
                    coords: coords.clone(),
                    active,
                },
            );
        }
    }
    cell_map
}

fn get_neighbors(coords: &(i32, i32, i32, i32), dimensions: &Dimensions) -> Vec<(i32, i32, i32, i32)> {
    let (x, y, z, w) = coords;
    let mut neighbors: Vec<(i32, i32, i32, i32)> = Vec::new();

    
    for nx in x - 1..x + 2 {
        for ny in y - 1..y + 2 {
            for nz in z - 1..z + 2 {
                if dimensions == &Dimensions::Four {
                    for nw in w - 1..w + 2 {
                        if !(&nx == x && &ny == y && &nz == z && &nw == w) {
                            neighbors.push((nx, ny, nz, nw));
                        }
                    }
                } else {
                    neighbors.push((nx, ny, nz, *w));
                }
            }
        }
    }

    neighbors
}

fn run_cycle(prev_state: HashMap<(i32, i32, i32, i32), Cell>, dimensions: Dimensions) -> HashMap<(i32, i32, i32, i32), Cell> {
    let mut new_state: HashMap<(i32, i32, i32, i32), Cell> = HashMap::new();
    let mut checked: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    // During a cycle, all cubes simultaneously change their state according to the following rules:
    for (coords, _cell) in prev_state.iter() {
        let mut to_check = get_neighbors(coords, &dimensions);
        to_check.push(coords.clone());

        let _checked = checked.to_owned();
        for to_check_coords in to_check.iter().filter(|c| !_checked.contains(c)) {
            let cell = match prev_state.get(to_check_coords) {
                Some(cell) => *cell,
                None => Cell {
                    coords: CellCoords {
                        x: to_check_coords.0,
                        y: to_check_coords.1,
                        z: to_check_coords.2,
                        w: to_check_coords.3
                    },
                    active: false,
                },
            };
            let all_neighbors = get_neighbors(to_check_coords, &dimensions);
            let active_neighbors = all_neighbors.iter().fold(0, |mut acc, curr| {
                acc += match prev_state.get(curr) {
                    Some(cell) => {
                        if cell.active {
                            1
                        } else {
                            0
                        }
                    }
                    None => 0,
                };
                acc
            });

            let mut copy = cell.to_owned();

            // - If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
            if cell.active && (active_neighbors < 2 || active_neighbors > 3) {
                copy.deactivate();
            // - If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
            } else if !cell.active && active_neighbors == 3 {
                copy.activate();
            }

            checked.insert(to_check_coords.clone());
            new_state.insert(*to_check_coords, copy);
        }
    }

    new_state
}

fn part_one() -> i32 {
    let mut cell_map = get_input_data();
    let mut count = 0;

    while count < 6 {
        cell_map = run_cycle(cell_map, Dimensions::Three);
        count += 1;
    }

    cell_map.iter().fold(0, |mut acc, (_, cell)| {
        if cell.active {
            acc += 1;
        }
        acc
    })
}

fn part_two() -> i32 {
    let mut cell_map = get_input_data();
    let mut count = 0;

    while count < 6 {
        cell_map = run_cycle(cell_map, Dimensions::Four);
        count += 1;
    }

    cell_map.iter().fold(0, |mut acc, (_, cell)| {
        if cell.active {
            acc += 1;
        }
        acc
    })
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
