use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Forest,
    Lumber,
}

#[derive(Clone)]
struct State {
    tiles: Vec<Vec<Tile>>,
}

impl State {
    fn step(&mut self) {
        let mut new_grid = self.tiles.clone();
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                let surrounding = self.surrounding((x, y));
                let new_tile = match tile {
                    Tile::Open => {
                        let num_trees = surrounding
                            .into_iter()
                            .filter(|t| *t == Tile::Forest)
                            .count();
                        if num_trees >= 3 {
                            Tile::Forest
                        } else {
                            Tile::Open
                        }
                    }
                    Tile::Forest => {
                        let num_lumber = surrounding
                            .into_iter()
                            .filter(|t| *t == Tile::Lumber)
                            .count();
                        if num_lumber >= 3 {
                            Tile::Lumber
                        } else {
                            Tile::Forest
                        }
                    }
                    Tile::Lumber => {
                        if surrounding.contains(&Tile::Lumber)
                            && surrounding.contains(&Tile::Forest)
                        {
                            Tile::Lumber
                        } else {
                            Tile::Open
                        }
                    }
                };
                new_grid[y][x] = new_tile;
            }
        }
        self.tiles = new_grid
    }

    fn surrounding(&self, position: (usize, usize)) -> Vec<Tile> {
        let (x, y) = position;
        let width = self.tiles[0].len();
        let height = self.tiles.len();
        [
            (x.saturating_sub(1), y.saturating_sub(1)),
            (x.saturating_sub(1), y),
            (x.saturating_sub(1), y + 1),
            (x, y.saturating_sub(1)),
            (x, y + 1),
            (x + 1, y.saturating_sub(1)),
            (x + 1, y),
            (x + 1, (y + 1)),
        ]
        .into_iter()
        .filter(|p| *p != position)
        .filter(|&(x, y)| x < width && y < height)
        .unique()
        .map(|(x, y)| self.tiles[y][x])
        .collect()
    }

    fn tree_count(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|t| **t == Tile::Forest)
            .count()
    }
    fn lumber_count(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|t| **t == Tile::Lumber)
            .count()
    }

    fn simulate(mut self, mut time: u32) -> usize {
        let mut seen = HashMap::new();
        while time > 0 {
            self.step();
            if let Some(prev) = seen.insert(self.tiles.clone(), time) {
                let cycle_len = prev - time;
                time %= cycle_len
            }
            time -= 1;
        }
        self.lumber_count() * self.tree_count()
    }
}

fn solve(input: &str) -> (usize, usize) {
    let tiles = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Open,
                    '|' => Tile::Forest,
                    '#' => Tile::Lumber,
                    _ => panic!("unrecognized character: {c}"),
                })
                .collect()
        })
        .collect();
    let state = State { tiles };
    let output_1 = state.clone().simulate(10);
    let output_2 = state.simulate(1000000000);
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
