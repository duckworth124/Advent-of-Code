use std::fs::read_to_string;

use itertools::{Itertools, iproduct};

#[derive(Clone, Copy, Debug)]
struct Grid([[bool; 100]; 100]);

impl Grid {
    fn step(&mut self, fix_corners: bool) {
        let mut new = [[false; 100]; 100];
        new.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .for_each(|(x, b)| *b = self.update((x, y)))
        });

        if fix_corners {
            for (x, y) in [(0, 0), (0, 99), (99, 0), (99, 99)] {
                new[y][x] = true;
            }
        }

        self.0 = new
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<bool> {
        self.0.get(y)?.get(x).copied()
    }

    fn update(&self, (x, y): (usize, usize)) -> bool {
        let surrounding = iproduct!(-1..=1, -1..=1)
            .filter(|&(x, y)| x != 0 || y != 0)
            .map(|(i, j)| (x as i32 + i, y as i32 + j))
            .map(|(i, j)| (i.try_into().ok(), j.try_into().ok()))
            .filter_map(|(i, j): (Option<usize>, Option<usize>)| i.zip(j))
            .filter_map(|p| self.get(p))
            .filter(|b| *b)
            .count();

        match surrounding {
            0..=1 | 4.. => false,
            2 => self.get((x, y)).unwrap_or_default(),
            3 => true,
        }
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let mut grid = Grid(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c == '#')
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec()
            .try_into()
            .unwrap(),
    );
    let mut grid_2 = grid;
    for _ in 0..100 {
        grid.step(false);
    }
    let output_1 = grid.0.iter().flatten().filter(|b| **b).count();
    for _ in 0..100 {
        grid_2.step(true);
    }
    let output_2 = grid_2.0.iter().flatten().filter(|b| **b).count();
    println!("part 1: {output_1} part 2: {output_2}")
}
