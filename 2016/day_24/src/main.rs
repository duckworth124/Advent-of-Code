use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use itertools::Itertools;
use priority_queue::PriorityQueue;

struct Grid {
    grid: Vec<Vec<bool>>,
    points_of_interest: Vec<(usize, usize)>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<bool>> = input
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        let mut points_of_interest = vec![(0, 0); 8];
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.char_indices() {
                if let Some(i) = c.to_digit(10) {
                    points_of_interest[i as usize] = (x, y)
                }
            }
        }

        debug_assert!(points_of_interest.iter().all(|&p| p != (0, 0)));

        Self {
            grid,
            points_of_interest,
        }
    }

    fn all_distances(&self) -> HashMap<(usize, usize), u32> {
        self.points_of_interest
            .iter()
            .copied()
            .enumerate()
            .permutations(2)
            .flat_map(|v| {
                let (s, start) = v[0];
                let (e, end) = v[1];
                let d = self.distance(start, end);
                [((s, e), d), ((e, s), d)]
            })
            .collect()
    }

    fn distance(&self, start: (usize, usize), end: (usize, usize)) -> u32 {
        let mut frontier = VecDeque::from([(start, 0)]);
        let mut visited = HashSet::new();
        let width = self.grid[0].len();
        let height = self.grid.len();
        loop {
            let (current, cost) = frontier.pop_front().unwrap();
            let (x, y) = current;
            if self.grid[y][x] {
                continue;
            }
            if !visited.insert(current) {
                continue;
            }
            if current == end {
                return cost;
            }
            frontier.extend(
                [
                    (x.saturating_sub(1), y),
                    (x, y.saturating_sub(1)),
                    (x + 1, y),
                    (x, y + 1),
                ]
                .into_iter()
                .filter(|&(x, y)| x < width && y < height)
                .map(|p| (p, cost + 1)),
            )
        }
    }
}

fn min_steps(distances: &HashMap<(usize, usize), u32>, return_to_0: bool) -> u32 {
    let mut frontier: PriorityQueue<(usize, u8), Reverse<u32>> =
        PriorityQueue::from_iter([((0, u8::MAX - 1), Reverse(0))]);
    let mut visited = HashSet::new();
    loop {
        let ((current, to_visit), Reverse(cost)) = frontier.pop().unwrap();

        if !visited.insert((current, to_visit)) {
            continue;
        }
        if to_visit == 0 {
            if return_to_0 {
                if current == 0 {
                    return cost;
                }
            } else {
                return cost;
            }
        }

        for next in 0..8 {
            if next == current {
                continue;
            }
            let d = distances[&(current, next)];
            let mut next_to_visit = to_visit;
            next_to_visit &= !(1 << next);
            frontier.push_increase((next, next_to_visit), Reverse(cost + d));
        }
    }
}

fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::parse(input);
    let distances = grid.all_distances();
    let output_1 = min_steps(&distances, false);
    let output_2 = min_steps(&distances, true);
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);

    println!("part 1: {output_1} part 2: {output_2}");
}
