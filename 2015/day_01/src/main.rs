use std::fs::read_to_string;

fn solve(path: &str) -> (i32, usize) {
    let input = read_to_string(path).unwrap();
    let processed: Vec<_> = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            _ => -1,
        })
        .collect();

    let output_1 = processed.iter().sum();
    let output_2 = processed
        .iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .position(|x| x < 0)
        .unwrap()
        + 1;

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
