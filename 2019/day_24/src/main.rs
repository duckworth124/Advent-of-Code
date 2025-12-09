use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, fs::read_to_string, iter};

#[derive(Clone, Copy)]
struct Grid([[bool; 5]; 5]);

impl Grid {
    fn step(self) -> Self {
        let mut new = self.0;
        (0..5).for_each(|y| {
            for x in 0..5 {
                let surrounding = self.count_surrounding((x, y));
                if surrounding == 1 {
                    new[y][x] = true;
                    continue;
                }
                if surrounding == 0 || surrounding >= 3 {
                    new[y][x] = false;
                    continue;
                }
                new[y][x] = !new[y][x]
            }
        });
        Self(new)
    }

    fn count_surrounding(self, (x, y): (usize, usize)) -> usize {
        [
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x, y.saturating_sub(1)),
            (x, y + 1),
        ]
        .into_iter()
        .filter(|&(x2, y2)| x2 < 5 && y2 < 5 && (x2, y2) != (x, y))
        .filter(|&(x, y)| self.0[y][x])
        .count()
    }

    fn biodiversity(self) -> u32 {
        self.0
            .iter()
            .flatten()
            .rev()
            .fold(0, |acc, &x| acc * 2 + x as u32)
    }
}

struct RecursiveGrid(HashSet<(u8, u8, i32)>);

impl RecursiveGrid {
    fn step(&mut self) {
        let new = self
            .0
            .iter()
            .flat_map(|&p| self.get_surrounding(p))
            .unique()
            .filter(|&p| self.is_alive_next(p))
            .collect();
        self.0 = new
    }

    fn is_alive_next(&self, p: (u8, u8, i32)) -> bool {
        let surrounding = self.count_surrounding(p);
        if surrounding == 1 {
            return true;
        }
        if surrounding == 0 || surrounding >= 3 {
            return false;
        }
        !self.0.contains(&p)
    }

    fn get_surrounding(&self, (x, y, d): (u8, u8, i32)) -> Vec<(u8, u8, i32)> {
        let mut output = vec![];
        if x > 0 {
            output.push((x - 1, y, d));
        } else {
            output.push((1, 2, d - 1));
        };
        if y > 0 {
            output.push((x, y - 1, d));
        } else {
            output.push((2, 1, d - 1));
        };
        if x < 4 {
            output.push((x + 1, y, d));
        } else {
            output.push((3, 2, d - 1));
        };
        if y < 4 {
            output.push((x, y + 1, d));
        } else {
            output.push((2, 3, d - 1));
        };

        if (x, y) == (1, 2) {
            output.extend((0..5).map(|y| (0, y, d + 1)));
        }
        if (x, y) == (3, 2) {
            output.extend((0..5).map(|y| (4, y, d + 1)));
        }
        if (x, y) == (2, 1) {
            output.extend((0..5).map(|x| (x, 0, d + 1)));
        }
        if (x, y) == (2, 3) {
            output.extend((0..5).map(|x| (x, 4, d + 1)));
        }

        output.retain(|&(x, y, _)| (x, y) != (2, 2));

        output
    }

    fn count_surrounding(&self, p: (u8, u8, i32)) -> usize {
        self.get_surrounding(p)
            .iter()
            .filter(|p| self.0.contains(p))
            .count()
    }
}

impl Display for RecursiveGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (d_min, d_max) = self
            .0
            .iter()
            .map(|(_, _, d)| *d)
            .minmax()
            .into_option()
            .unwrap();

        for d in d_min..=d_max {
            writeln!(f, "level {d}:")?;
            for y in 0..5 {
                for x in 0..5 {
                    if self.0.contains(&(x, y, d)) {
                        write!(f, "#")?
                    } else {
                        write!(f, ".")?
                    }
                }
                writeln!(f)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

fn solve(input: &str) -> (u32, usize) {
    let grid = Grid(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c == '#')
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    );
    let mut seen = HashSet::new();
    let output_1 = iter::successors(Some(grid), |x| Some(x.step()))
        .find(|x| !seen.insert(x.0))
        .unwrap()
        .biodiversity();

    let mut recursive_grid = RecursiveGrid(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.char_indices().map(move |(x, c)| (x, y, c == '#')))
            .filter(|&(_, _, c)| c)
            .map(|(x, y, _)| (x as u8, y as u8, 0))
            .collect(),
    );
    for _ in 0..200 {
        recursive_grid.step();
    }
    let output_2 = recursive_grid.0.len();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
