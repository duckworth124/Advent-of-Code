use itertools::Itertools;
use std::{cmp::Ordering, fs::read_to_string};

fn is_ordered(list: &[u32], rules: &[(u32, u32)]) -> bool {
    rules
        .iter()
        .map(|(l, r)| {
            (
                list.iter().position(|x| x == l),
                list.iter().position(|x| x == r),
            )
        })
        .flat_map(|(l, r)| l.zip(r))
        .all(|(l, r)| l < r)
}

fn sort(list: &mut [u32], rules: &[(u32, u32)]) {
    list.sort_by(|x, y| {
        if x == y {
            Ordering::Equal
        } else if rules.iter().contains(&(*x, *y)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
}

fn solve(path: &str) -> (u32, u32) {
    let input = read_to_string(path).unwrap();
    let rules = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split('|')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut lists = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect_vec())
        .collect_vec();

    let output_1 = lists
        .iter()
        .filter(|l| is_ordered(l, &rules))
        .map(|l| l[l.len() / 2])
        .sum();

    lists.iter_mut().for_each(|l| sort(l, &rules));
    let ouptut_2 = lists.iter().map(|l| l[l.len() / 2]).sum::<u32>() - output_1;

    (output_1, ouptut_2)
}

fn main() {
    let (output_1, ouptut_2) = solve("input");
    println!("part 1: {output_1} part 2: {ouptut_2}")
}
