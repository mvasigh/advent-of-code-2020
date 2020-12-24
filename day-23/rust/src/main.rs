fn slide_to_val(vector: &Vec<usize>, val: &usize) -> Vec<usize> {
    let index = vector
        .iter()
        .position(|v| v == val)
        .expect("Value does not exist in this vector");

    let mut new_vector = vector.to_owned();
    let len = new_vector.len();

    for i in 0..len {
        new_vector[i] = vector[(i + index) % len];
    }
    new_vector
}

fn run_moves(mut cups: Vec<usize>, times: u16) -> Vec<usize> {
    let mut current_cup = cups[0];
    let mut count = 0;

    let max = cups.iter().fold(0 as usize, |acc, curr| if curr > &acc { *curr } else { acc });
    let min = cups.iter().fold(10 as usize, |acc, curr| if curr < &acc { *curr } else { acc });

    while count < times {
        // The crab picks up the three cups that are immediately clockwise of the current cup. They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let partial = cups.drain(1..4).collect::<Vec<usize>>();
        let mut slice = cups[1..4].to_vec();
        slice.reverse();

        // The crab selects a destination cup: the cup with a label equal to the current cup's label minus one. 
        let mut destination = if current_cup <= min { max } else { current_cup - 1 };
        
        loop {
            // If this would select one of the cups that was just picked up, the crab will keep subtracting one until it finds a cup that wasn't just picked up. 
            if partial.contains(&destination) && destination >= min {
                break;
            }
            
            // If at any point in this process the value goes below the lowest value on any cup's label, it wraps around to the highest value on any cup's label instead.
            if destination < min {
                destination = max;
                continue;
            }

            // ... keep subtracting one
            destination -= 1;
        }

        let dest_i = partial.iter().position(|c| *c == destination).expect("Could not get index for destination");

        // The crab places the cups it just picked up so that they are immediately clockwise of the destination cup. They keep the same order as when they were picked up.
        cups = slice.iter().fold(partial, |mut acc, cup| {
            acc.insert(dest_i, *cup);
            acc
        });

        // The crab selects a new current cup: the cup which is immediately clockwise of the current cup
        current_cup = cups[dest_i + 1];
    }

    cups
}

fn part_one() -> String {
    let mut starting_cups = String::from("643719258")
        .chars()
        .map(|el| el.to_string().parse::<usize>().expect("Could not parse"))
        .collect::<Vec<usize>>();

    let finished_cups = slide_to_val(&run_moves(starting_cups, 100), &1)
        .iter()
        .skip(1)
        .fold(String::new(), |mut acc, curr| {
            acc.push_str(&curr.to_string());
            acc
        });

    finished_cups
}

fn main() {
    println!("Part 1: {}", part_one());
}
