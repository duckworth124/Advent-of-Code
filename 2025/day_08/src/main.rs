use itertools::Itertools;
use std::{fs::read_to_string, time::Instant};

#[derive(Clone, Copy)]
struct Point([u64; 3]);

impl Point {
    fn distance_squared(self, other: Self) -> u64 {
        self.0
            .into_iter()
            .zip(other.0)
            .map(|(x, y)| x.abs_diff(y).pow(2))
            .sum()
    }
}

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }
        let output = self.find(self.parents[x]);
        self.parents[x] = output;
        output
    }

    fn join(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        self.parents[x_root] = y_root
    }

    fn set_sizes(&mut self) -> Vec<usize> {
        let mut output = vec![0; self.parents.len()];
        for i in 0..self.parents.len() {
            let x = self.parents[i];
            output[self.find(x)] += 1;
        }
        output
    }

    fn all_connected(&mut self) -> bool {
        (0..self.parents.len()).map(|x| self.find(x)).all_equal()
    }
}

fn solve(input: &str) -> (usize, u64) {
    let elements: Vec<Point> = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .map(Point)
        .collect();
    let num_points = elements.len();
    let mut union_find = UnionFind {
        parents: (0..num_points).collect(),
    };
    for (x, y) in (0..num_points)
        .tuple_combinations()
        .k_smallest_by_key(1000, |&(x, y)| elements[x].distance_squared(elements[y]))
    {
        union_find.join(x, y);
    }
    let output_1 = union_find.set_sizes().into_iter().k_largest(3).product();

    let mut output_2 = None;
    for (x, y) in (0..num_points)
        .tuple_combinations()
        .sorted_unstable_by_key(|&(x, y)| elements[x].distance_squared(elements[y]))
    {
        union_find.join(x, y);
        if union_find.all_connected() {
            output_2 = Some(elements[x].0[0] * elements[y].0[0]);
            break;
        }
    }
    let output_2 = output_2.unwrap();
    (output_1, output_2)
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32());
}
