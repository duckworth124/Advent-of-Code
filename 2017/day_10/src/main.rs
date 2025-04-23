use std::fs::read_to_string;

struct State {
    numbers: Vec<u32>,
    skip_size: usize,
    start: usize,
}

impl State {
    fn update(&mut self, length: usize) {
        self.numbers[..length].reverse();
        let rotate_length = (length + self.skip_size) % 256;
        self.numbers.rotate_left(rotate_length);
        self.start += 256 - rotate_length;
        self.start %= 256;
        self.skip_size += 1;
    }
}

fn knot_hash(input: &str) -> Vec<u8> {
    let mut state = State {
        numbers: (0..256).collect(),
        skip_size: 0,
        start: 0,
    };
    for _ in 0..64 {
        for length in input.trim().bytes().chain([17, 31, 73, 47, 23]) {
            state.update(length as usize);
        }
    }

    state.numbers.rotate_left(state.start);

    let dense_hash: Vec<u8> = state
        .numbers
        .chunks(16)
        .into_iter()
        .map(|c| c.iter().fold(0, |acc, x| acc ^ x))
        .map(|n| n as u8)
        .collect();

    dense_hash
}

fn solve(input: &str) -> (u32, String) {
    let mut state = State {
        numbers: (0..256).collect(),
        skip_size: 0,
        start: 0,
    };
    for length in input.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
        state.update(length);
    }
    state.numbers.rotate_left(state.start);
    let output_1 = state.numbers[0] * state.numbers[1];

    let hash = knot_hash(input.trim());

    let output_2: String = hash.into_iter().map(|n| format!("{n:02x}")).collect();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}

//6979100dd93bea7b653b008f7a1071b4
