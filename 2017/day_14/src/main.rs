use std::fs::read_to_string;

use reunion::{UnionFind, UnionFindTrait};

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

fn bits(n: u8) -> Vec<bool> {
    (0..8)
        .map(|n| 1 << n)
        .map(|x| n & x)
        .map(|x| x > 0)
        .rev()
        .collect()
}

fn solve(input: &str) -> (u32, usize) {
    let grid: Vec<Vec<bool>> = (0..128)
        .map(|i| format!("{}-{}", input.trim(), i))
        .map(|s| knot_hash(&s))
        .map(|v| v.into_iter().flat_map(bits).collect())
        .collect();

    let output_1 = grid.iter().flat_map(|v| v.iter().map(|&b| b as u32)).sum();

    let mut squares = UnionFind::new();
    for i in 0..128 {
        for j in 0..128 {
            if !grid[i][j] {
                continue;
            }
            squares.union((i, j), (i, j));
            for (i2, j2) in [
                (i.saturating_sub(1), j),
                ((i + 1).min(127), j),
                (i, j.saturating_sub(1)),
                (i, (j + 1).min(127)),
            ] {
                if grid[i2][j2] {
                    squares.union((i, j), (i2, j2));
                }
            }
        }
    }

    println!("{}", squares.size());

    let output_2 = squares.subsets().len();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
