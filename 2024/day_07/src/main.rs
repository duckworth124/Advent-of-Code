use itertools::Itertools;
use std::fs::read_to_string;

fn is_reachable(operands: &[u64], target: u64, allow_concatenation: bool) -> bool {
    if operands.len() == 1 {
        return operands[0] == target;
    }

    let last = *operands.last().unwrap();
    if allow_concatenation
        && target.to_string().ends_with(&last.to_string())
        && is_reachable(
            &operands[..operands.len() - 1],
            target
                .to_string()
                .strip_suffix(&last.to_string())
                .unwrap()
                .parse()
                .unwrap_or(0),
            allow_concatenation,
        )
    {
        return true;
    }
    if target % last == 0
        && is_reachable(
            &operands[..operands.len() - 1],
            target / last,
            allow_concatenation,
        )
    {
        return true;
    }

    if target >= last
        && is_reachable(
            &operands[..operands.len() - 1],
            target - last,
            allow_concatenation,
        )
    {
        return true;
    }

    false
}

fn solve(path: &str) -> (u64, u64) {
    let input = read_to_string(path).unwrap();
    let lists = input
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(s, s2)| {
            (
                s.parse().unwrap(),
                s2.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let output_1 = lists
        .iter()
        .filter(|(target, operands)| is_reachable(operands, *target, false))
        .map(|(t, _)| t)
        .sum();

    let output_2 = lists
        .iter()
        .filter(|(target, operands)| is_reachable(operands, *target, true))
        .map(|(t, _)| t)
        .sum();

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
