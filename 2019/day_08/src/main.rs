use std::fs::read_to_string;

use itertools::Itertools;

fn solve(input: &str) -> usize {
    let layers = input
        .trim()
        .chars()
        .chunks(25 * 6)
        .into_iter()
        .map(|c| {
            c.chunks(25)
                .into_iter()
                .map(|c| c.collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let layer = layers
        .iter()
        .map(|v| v.iter().flatten().collect_vec())
        .min_by_key(|v| v.iter().filter(|c| c == &&&'0').count())
        .unwrap();

    println!("part 2:");
    for row in 0..6 {
        for col in 0..25 {
            for layer in &layers {
                let pixel = layer[row][col];
                if pixel == '2' {
                    continue;
                }
                if pixel == '1' {
                    print!("#")
                } else {
                    print!(".")
                }
                break;
            }
        }
        println!()
    }
    println!();

    layer.iter().filter(|&&&c| c == '1').count() * layer.iter().filter(|&&&c| c == '2').count()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = solve(&input);
    println!("part 1: {output_1}")
}
