use std::{
    cmp::{max, min},
    fs::read_to_string,
    mem::swap,
};

fn gcd(x: u64, y: u64) -> u64 {
    let mut u = max(x, y);
    let mut l = min(x, y);

    while u % l != 0 {
        u %= l;
        swap(&mut u, &mut l);
    }
    l
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn merge_constraints((q1, r1): (u64, u64), (q2, r2): (u64, u64)) -> (u64, u64) {
    let q = lcm(q1, q2);
    let r = (0..).map(|n| r1 + n * q1).find(|&x| x % q2 == r2).unwrap() % q;

    (q, r)
}

fn start_of_consecutive_departures(ids: &[Option<u64>]) -> u64 {
    ids.iter()
        .enumerate()
        .filter_map(|(i, &o)| o.zip(Some(i as u64)))
        .map(|(q, r)| (q, r % q))
        .map(|(q, r)| (q, q - r))
        .reduce(merge_constraints)
        .unwrap()
        .1
}

pub fn solve(path: &str) -> (u64, u64) {
    let input = read_to_string(path).unwrap();
    let earliest: u64 = input.lines().next().unwrap().parse().unwrap();
    let ids: Vec<Option<u64>> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse().ok())
        .collect();

    let (first_departure, id) = (earliest..)
        .find_map(|t| {
            ids.iter()
                .flatten()
                .find_map(|&i| Some((t, i)).filter(|_| t % i == 0))
        })
        .unwrap();

    let output_1 = (first_departure - earliest) * id;

    let output_2 = start_of_consecutive_departures(&ids);
    (output_1, output_2)
}
