const PUBLIC_KEYS: (u64, u64) = (8252394, 6269621);

fn transform(val: u64, subject: &u64) -> u64 {
    (val * subject) % 20201227
}

fn find_loop_size(public_key: &u64) -> u64 {
    let mut val = 1;
    let mut loop_size = 0;

    while val != *public_key {
        val = transform(val, &7);
        loop_size += 1;
    }

    loop_size
}

fn part_one() -> u64 {
    let (card_public_key, door_public_key) = PUBLIC_KEYS;
    let card_loop_size = find_loop_size(&card_public_key);

    let mut encryption_key: u64 = 1;

    for _i in 0..card_loop_size {
        encryption_key = transform(encryption_key, &door_public_key);
    }

    encryption_key
}


fn main() {
    println!("Part 1: {}", part_one());
}
