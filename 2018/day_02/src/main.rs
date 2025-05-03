use std::fs::read_to_string;

use itertools::Itertools;

fn contains_double_or_triple(line: &str) -> (bool, bool) {
    let mut has_double = false;
    let mut has_triple = false;
    line.chars().counts().into_values().for_each(|n| {
        if n == 3 {
            has_triple = true
        } else if n == 2 {
            has_double = true
        }
    });
    (has_double, has_triple)
}

fn common_letters(x: &str, y: &str) -> Option<String> {
    if x.chars().zip(y.chars()).filter(|(l, r)| l != r).count() == 1 {
        Some(
            x.chars()
                .zip(y.chars())
                .filter(|(l, r)| l == r)
                .map(|(l, _)| l)
                .collect(),
        )
    } else {
        None
    }
}

fn solve(input: &str) -> (usize, String) {
    let (doubles, triples) = input.lines().map(contains_double_or_triple).fold(
        (0, 0),
        |(mut doubles, mut triples), (has_double, has_triple)| {
            if has_double {
                doubles += 1;
            }
            if has_triple {
                triples += 1
            }
            (doubles, triples)
        },
    );

    let output_1 = doubles * triples;
    let output_2 = input
        .lines()
        .permutations(2)
        .find_map(|v| common_letters(v[0], v[1]))
        .unwrap();

    (output_1, output_2)
}
fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
