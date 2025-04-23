use reunion::{UnionFind, UnionFindTrait};
use std::fs::read_to_string;

fn solve(input: &str) -> (usize, usize) {
    let mut programs = UnionFind::<u32>::new();
    for line in input.lines() {
        let (l, r) = line.split_once("<->").unwrap();
        let id = l.trim().parse().unwrap();
        for connection in r.trim().split(", ").map(|s| s.parse().unwrap()) {
            programs.union(id, connection);
        }
    }

    let output_1 = programs
        .subsets()
        .into_iter()
        .find(|s| s.contains(&0))
        .unwrap()
        .len();

    let output_2 = programs.subsets().len();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
