use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

use priority_queue::PriorityQueue;

const fn is_blocked((x, y): (u32, u32), favourite_number: u32) -> bool {
    let val = x * x + 3 * x + 2 * x * y + y + y * y + favourite_number;
    !val.count_ones().is_multiple_of(2)
}

const fn heuristic((x, y): (u32, u32)) -> u32 {
    x.abs_diff(31) + y.abs_diff(39)
}

fn solve(input: &str) -> (u32, usize) {
    let favourite_number: u32 = input.trim().parse().unwrap();
    let mut frontier: PriorityQueue<((u32, u32), u32), Reverse<u32>> =
        PriorityQueue::from_iter([(((1, 1), 0), Reverse(0))]);
    let mut visited = HashSet::new();

    let output_1 = loop {
        let (((x, y), time), _) = frontier.pop().unwrap();
        if is_blocked((x, y), favourite_number) {
            continue;
        }
        if !visited.insert((x, y)) {
            continue;
        }

        if (x, y) == (31, 39) {
            break time;
        }

        [
            (x + 1, y),
            (x, y + 1),
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
        ]
        .into_iter()
        .for_each(|p| {
            frontier.push_increase((p, time + 1), Reverse(time + 1 + heuristic(p)));
        });
    };

    let mut can_reach = HashSet::new();
    let mut frontier = VecDeque::from([((1, 1), 0)]);

    let output_2 = loop {
        let ((x, y), steps) = frontier.pop_front().unwrap();
        if steps > 50 {
            break can_reach.len();
        }
        if is_blocked((x, y), favourite_number) {
            continue;
        }
        if !can_reach.insert((x, y)) {
            continue;
        }
        [
            (x + 1, y),
            (x, y + 1),
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
        ]
        .into_iter()
        .for_each(|p| {
            frontier.push_back((p, steps + 1));
        });
    };

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
