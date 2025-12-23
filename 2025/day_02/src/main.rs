use std::{collections::HashSet, fs::read_to_string, iter, time::Instant};

fn invalid_ids(low: u64, high: u64) -> impl Iterator<Item = u64> {
    (1..)
        .map(|x| x * (10_u64.pow(num_digits(x)) + 1))
        .skip_while(move |&x| x < low)
        .take_while(move |&x| x <= high)
}

fn invalid_ids_many_repeats(low: u64, high: u64) -> HashSet<u64> {
    (1..)
        .take_while(|&x| x * 10_u64.pow(num_digits(x)) + x <= high)
        .flat_map(|x| {
            iter::successors(Some(x), move |&n| {
                Some(
                    n.saturating_mul(10_u64.saturating_pow(num_digits(x)))
                        .saturating_add(x),
                )
            })
            .skip(1)
            .skip_while(|&x| x < low)
            .take_while(|&x| x <= high)
        })
        .collect()
}

const fn num_digits(x: u64) -> u32 {
    x.ilog10() + 1
}

fn solve(input: &str) -> (u64, u64) {
    let mut output_1 = 0;
    let mut output_2 = 0;
    for (low, high) in input
        .split(',')
        .map(|s| s.split_once('-').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
    {
        output_1 += invalid_ids(low, high).sum::<u64>();
        output_2 += invalid_ids_many_repeats(low, high).into_iter().sum::<u64>();
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
