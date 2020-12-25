const CARD_PUB_KEY: usize = 17773298;
const DOOR_PUB_KEY: usize = 15530095;

fn iterate(value: usize, subject: usize) -> usize {
    (value * subject) % 20201227
}

fn main() {
    let mut value = 1;
    let mut card_loop_size = 1;
    loop {
        value = iterate(value, 7);
        if value == CARD_PUB_KEY {
            break;
        }
        card_loop_size += 1;
    }

    let mut encryption_key = 1;
    for _ in 0..card_loop_size {
        encryption_key = iterate(encryption_key, DOOR_PUB_KEY);
    }

    println!("Part 1: {}", encryption_key);
}