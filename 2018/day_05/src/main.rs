use std::fs::read_to_string;

fn reduce(mut polymer: Vec<char>) -> Vec<char> {
    let mut i = 0;
    while i + 1 < polymer.len() {
        let c_1 = polymer[i];
        let c_2 = polymer[i + 1];
        if c_1.eq_ignore_ascii_case(&c_2) && c_1.is_ascii_uppercase() != c_2.is_ascii_uppercase() {
            polymer.remove(i);
            polymer.remove(i);
            i = i.saturating_sub(1);
            continue;
        }
        i += 1;
    }
    polymer
}

fn solve(input: &str) -> (usize, usize) {
    let polymer: Vec<char> = reduce(input.chars().collect());
    let output_1 = polymer.len();
    let output_2 = ('a'..='z')
        .map(|c| {
            polymer
                .iter()
                .copied()
                .filter(|&c_2| !c.eq_ignore_ascii_case(&c_2))
                .collect::<Vec<char>>()
        })
        .map(reduce)
        .map(|v| v.len())
        .min()
        .unwrap();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}")
}
