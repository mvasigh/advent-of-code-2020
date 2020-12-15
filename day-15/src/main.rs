use std::collections::HashMap;

fn get_input_data() -> Vec<i32> {
    vec![9, 12, 1, 4, 17, 0, 18]
}

fn get_nth_number(n: usize, seed: &Vec<i32>) -> i32 {
    let mut count: usize = 0;
    let mut used_indices: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut last_num = 0;

    while count < n {
        let is_seed = count < seed.len();
        let num = if is_seed {
            *seed.get(count).unwrap()
        } else {
            let indices = used_indices.get(&last_num).unwrap();
            if indices.len() > 1 {
                let last = *indices.get(indices.len() - 1).unwrap() as i32;
                let second_last = *indices.get(indices.len() - 2).unwrap() as i32;
                last - second_last
            } else {
                0
            }
        };

        let entry = used_indices.entry(num).or_insert(Vec::new());
        entry.push(count);
        
        last_num = num;
        count += 1;
    }

    last_num
}

fn part_one() -> i32 {
    let input_data = get_input_data();
    get_nth_number(2020, &input_data)
}

fn main() {
    println!("Part 1: {}", part_one());
}
