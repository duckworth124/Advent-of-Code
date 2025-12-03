use std::{cmp::Reverse, fs::read_to_string, time::Instant};

fn max_joltage(mut line: &str, num_digits: usize) -> u64 {
    let mut output = 0;
    for i in 0..num_digits {
        let (first_pos, first) = line
            .char_indices()
            .take(line.len() + i + 1 - num_digits)
            .max_by_key(|&(i, c)| (c, Reverse(i)))
            .unwrap();
        line = &line[first_pos + 1..];
        output *= 10;
        output += first as u64 - '0' as u64
    }
    output
}

fn solve(input: &str) -> (u64, u64) {
    let mut output_1 = 0;
    let mut output_2 = 0;
    for line in input.lines() {
        output_1 += max_joltage(line, 2);
        output_2 += max_joltage(line, 12);
    }
    (output_1, output_2)
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
