use std::iter::successors;

const fn next_code(prev_code: u64) -> u64 {
    (prev_code * 252533) % 33554393
}

fn code_sequence() -> impl Iterator<Item = u64> {
    successors(Some(20151125), |p| Some(next_code(*p)))
}

const fn triangle_number(n: u32) -> u32 {
    n * (n - 1) / 2
}

const fn code_number(row: u32, col: u32) -> usize {
    let diagonal_number = row + col;
    (triangle_number(diagonal_number - 1) + col - 1) as usize
}

fn solve() -> u64 {
    let (row, col) = (3010, 3019);
    code_sequence().nth(code_number(row, col)).unwrap()
}

fn main() {
    let output = solve();
    println!("part 1: {output}")
}
