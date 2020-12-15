use std::collections::HashMap;

fn get_input_data() -> Vec<usize> {
    vec![9, 12, 1, 4, 17, 0, 18]
}

fn get_nth_number(n: usize, seed: &Vec<usize>) -> usize {
    let mut count: usize = 0;
    let mut used_indices: HashMap<usize, (Option<usize>, Option<usize>)> = HashMap::new();
    let mut last_num = 0;
    let seed_len = seed.len();

    while count < n {
        let is_seed = count < seed_len;
        let num = if is_seed {
            *seed.get(count).unwrap()
        } else {
            let indices = used_indices.get(&last_num).unwrap();
            if indices.0.is_none() {
                0
            } else {
                indices.1.unwrap() - indices.0.unwrap()
            }
        };

        let entry = used_indices.entry(num).or_insert((None, None));
        entry.0 = entry.1;
        entry.1 = Some(count);

        last_num = num;
        count += 1;
    }

    last_num
}

fn part_one() -> usize {
    let input_data = get_input_data();
    get_nth_number(2020, &input_data)
}

fn part_two() -> usize {
    let input_data = get_input_data();
    get_nth_number(30000000, &input_data)
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
