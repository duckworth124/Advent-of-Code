use std::{fs::read_to_string, time::Instant};

fn is_monotone(data: &[u32]) -> bool {
    data.windows(2).all(|v| v[0] < v[1]) || data.windows(2).all(|v| v[0] > v[1])
}

fn is_gradual(data: &[u32]) -> bool {
    data.windows(2)
        .all(|v| (1..=3).contains(&v[0].abs_diff(v[1])))
}

fn is_safe(data: &[u32]) -> bool {
    is_monotone(data) && is_gradual(data)
}

fn process_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let data = process_input(&input);
    let output_1 = data.iter().filter(|x| is_safe(x)).count();
    let output_2 = data
        .iter()
        .filter(|x| {
            (0..x.len())
                .map(|i| {
                    let mut data = x.to_vec();
                    data.remove(i);
                    data
                })
                .any(|d| is_safe(&d))
        })
        .count();

    (output_1, output_2)
}

fn main() {
    let now = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", now.elapsed().as_secs_f64())
}
