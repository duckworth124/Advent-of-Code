use std::{fs::read_to_string, time::Instant};

fn solve(input: &str) -> (usize, u64) {
    let (l, r) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(u64, u64)> = l
        .lines()
        .map(|s| s.split_once('-').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    ranges.sort_unstable();

    let mut i = 0;
    while i < ranges.len() - 1 {
        let (_, high1) = ranges[i];
        let (low2, high2) = ranges[i + 1];
        if low2 <= high1 {
            ranges[i].1 = high1.max(high2);
            ranges.remove(i + 1);
            continue;
        }
        i += 1
    }

    let output_1 = r
        .lines()
        .map(|s| s.parse().unwrap())
        .filter(|x: &u64| {
            ranges
                .iter()
                .skip_while(|(_, r)| x > r)
                .take_while(|(l, _)| x >= l)
                .next()
                .is_some()
        })
        .count();
    let output_2 = ranges.iter().map(|(l, r)| r - l + 1).sum();
    (output_1, output_2)
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
