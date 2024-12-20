use itertools::Itertools;
use std::{fs::read_to_string, time::Instant};

fn process_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

fn solve(path: &str) -> (u32, u32) {
    let input = read_to_string(path).unwrap();
    let (list1, list2) = process_input(&input);
    let output_1 = list1
        .iter()
        .sorted()
        .zip(list2.iter().sorted())
        .map(|(x, y)| x.abs_diff(*y))
        .sum();

    let output_2 = list1
        .iter()
        .map(|x| *x * list2.iter().filter(|y| *x == **y).count() as u32)
        .sum();

    (output_1, output_2)
}

fn main() {
    let now = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", now.elapsed().as_secs_f64())
}
