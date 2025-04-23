use std::fs::read_to_string;

fn solve(input: &str) -> (u32, u32) {
    let security: Vec<(u32, u32, u32)> = input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .map(|(depth, range): (u32, u32)| (depth, range, (range - 1) * 2))
        .collect();

    let output_1 = security
        .iter()
        .copied()
        .filter(|(depth, _, cycle_length)| depth % cycle_length == 0)
        .map(|(depth, range, _)| depth * range)
        .sum();

    let output_2 = (0..)
        .find(|d| {
            security
                .iter()
                .map(|(depth, _, cycle)| (depth, cycle))
                .map(|(depth, cycle)| (depth + d, cycle))
                .all(|(x, y)| x % y != 0)
        })
        .unwrap();

    (output_1, output_2)
}
fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
