use itertools::Itertools;
use std::iter;

fn get_digits(n: u32) -> Vec<u32> {
    iter::successors(Some(n), |&m| Some(m / 10))
        .take_while(|&m| m > 0)
        .map(|m| m % 10)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn is_increasing(digits: &[u32]) -> bool {
    digits.iter().tuple_windows().all(|(x, y)| x <= y)
}

fn has_double(digits: &[u32]) -> bool {
    digits.iter().tuple_windows().any(|(x, y)| x == y)
}

fn has_only_double(digits: &[u32]) -> bool {
    digits
        .iter()
        .chunk_by(|x| **x)
        .into_iter()
        .any(|(_, c)| c.count() == 2)
}

fn is_valid(n: u32) -> bool {
    let digits = get_digits(n);
    is_increasing(&digits) && has_double(&digits)
}

fn is_valid_2(n: u32) -> bool {
    let digits = get_digits(n);
    is_increasing(&digits) && has_only_double(&digits)
}

fn main() {
    let output_1 = (138307..=654504).filter(|n| is_valid(*n)).count();
    let output_2 = (138307..=654504).filter(|n| is_valid_2(*n)).count();

    println!("part 1: {output_1} part 2: {output_2}")
}
