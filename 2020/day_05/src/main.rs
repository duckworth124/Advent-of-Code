use std::fs::read_to_string;

fn seat_id(input: &str) -> u16 {
    input[..7]
        .chars()
        .map(|c| c == 'B')
        .chain(input[7..].chars().map(|c| c == 'R'))
        .fold(0, |acc, b| acc * 2 + b as u16)
}

fn solve(path: &str) -> (u16, u16) {
    let input = read_to_string(path).unwrap();
    let mut values: Vec<u16> = input.lines().map(seat_id).collect();
    values.sort_unstable();
    let output_1 = *values.last().unwrap();
    let output_2 = values.windows(2).find(|v| v[1] > v[0] + 1).unwrap()[0] + 1;
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
