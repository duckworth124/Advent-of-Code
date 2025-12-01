use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();
    let keys: Vec<u64> = input.lines().map(|s| s.parse().unwrap()).collect();
    let door_key = keys[0];
    let card_key = keys[1];
    let mut current = 1;
    let mut loop_count = 0;
    while current != door_key {
        loop_count += 1;
        current *= 7;
        current %= 20201227;
    }

    current = 1;
    for _ in 0..loop_count {
        current *= card_key;
        current %= 20201227;
    }

    let output_1 = current;
    println!("part 1: {output_1}")
}
