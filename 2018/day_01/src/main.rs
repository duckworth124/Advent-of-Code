use std::{collections::HashSet, fs::read_to_string};

fn solve(input: &str) -> (i32, i32) {
    let changes: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let output_1 = changes.iter().sum();
    let mut seen = HashSet::new();
    let output_2 = changes
        .into_iter()
        .cycle()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .find(|x| !seen.insert(*x))
        .unwrap();

    (output_1, output_2)
}
fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
