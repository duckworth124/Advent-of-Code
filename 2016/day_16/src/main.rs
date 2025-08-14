use std::fs::read_to_string;

use itertools::Itertools;

fn expand(input: &str) -> String {
    let a = input;
    let b: String = a
        .chars()
        .rev()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect();

    format!("{a}0{b}")
}

fn get_checksum(init: &str, len: usize) -> String {
    let mut data = init.trim().to_string();
    while data.len() < len {
        data = expand(&data)
    }
    data = data[..len].to_string();
    let mut output = data;
    while output.len() % 2 == 0 {
        output = output
            .chars()
            .chunks(2)
            .into_iter()
            .map(|mut v| {
                if v.next().unwrap() == v.next().unwrap() {
                    '1'
                } else {
                    '0'
                }
            })
            .collect();
    }

    output
}

fn solve(input: &str) -> (String, String) {
    let output_1 = get_checksum(input, 272);
    let output_2 = get_checksum(input, 35651584);
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
