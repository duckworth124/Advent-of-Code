use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

struct UnionFind(Vec<Option<usize>>);

impl UnionFind {
    fn add_connection(&mut self, x: usize, y: usize) {
        let x_root = self.find_compress(x);
        let y_root = self.find_compress(y);
        if x_root == y_root {
            return;
        }
        self.0[y_root] = Some(x_root);
    }

    fn find_compress(&mut self, i: usize) -> usize {
        match self.0[i] {
            None => i,
            Some(j) => {
                let output = self.find_compress(j);
                self.0[i] = Some(output);
                output
            }
        }
    }

    fn find(&self, i: usize) -> usize {
        self.0[i].map_or(i, |j| self.find(j))
    }

    fn num_sets(&self) -> usize {
        (0..self.0.len())
            .map(|i| self.find(i))
            .collect::<HashSet<_>>()
            .len()
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let coords: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|s| {
            s.split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let len = coords.len();
    let mut union_find = UnionFind(vec![None; len]);
    for i in 0..len {
        for j in (i + 1)..len {
            let (x1, y1, z1, w1) = coords[i];
            let (x2, y2, z2, w2) = coords[j];
            let d = x1.abs_diff(x2) + y1.abs_diff(y2) + z1.abs_diff(z2) + w1.abs_diff(w2);
            if d <= 3 {
                union_find.add_connection(i, j);
            }
        }
    }
    let output_1 = union_find.num_sets();
    println!("part 1: {output_1}")
}
