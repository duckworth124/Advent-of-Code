use std::fs::read_to_string;

use regex::Regex;

fn mul_sum(input: &str) -> u32 {
    let pat = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    pat.captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum()
}

fn mul_sum_cond(input: &str) -> u32 {
    input
        .split("do()")
        .flat_map(|s| s.split("don't()").next())
        .map(mul_sum)
        .sum()
}

fn solve(path: &str) -> (u32, u32) {
    let input = read_to_string(path).unwrap();
    let output_1 = mul_sum(&input);
    let output_2 = mul_sum_cond(&input);
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
}
