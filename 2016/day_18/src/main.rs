use itertools::{Itertools, chain};
use std::fs::read_to_string;

fn next_row(prev_row: &[bool]) -> Vec<bool> {
    chain!(&[false], prev_row, &[false])
        .copied()
        .tuple_windows()
        .map(|(x, _y, z)| x ^ z)
        .collect()
}

fn solve(input: &str) -> (usize, usize) {
    let mut rows: Vec<Vec<bool>> = vec![input.trim().chars().map(|c| c == '^').collect()];
    for _ in 0..39 {
        let new = next_row(rows.last().unwrap());
        rows.push(new);
    }

    let output_1 = rows.iter().flatten().copied().filter(|b| !b).count();

    for _ in 0..400_000 - 40 {
        let new = next_row(rows.last().unwrap());
        rows.push(new);
    }

    let output_2 = rows.iter().flatten().copied().filter(|b| !b).count();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
