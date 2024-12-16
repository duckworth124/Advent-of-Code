use std::fs::read_to_string;

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn get_adjacent_points(&self) -> Vec<Self> {
        let min_x = self.x.saturating_sub(1);
        let max_x = self.x + 1;
        let min_y = self.y.saturating_sub(1);
        let max_y = self.y + 1;

        (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .map(|(x, y)| Position::new(x, y))
            .filter(|p| p != self)
            .collect_vec()
    }
}

#[derive(Clone)]
struct Octopus {
    energy: u32,
    flashed: bool,
}

impl Octopus {
    fn new(c: char) -> Self {
        let energy = c.to_digit(10).unwrap();
        let flashed = false;
        Self { energy, flashed }
    }
}

#[derive(Clone)]
struct Grid(Vec<Vec<Octopus>>);

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(Octopus::new).collect())
            .collect();
        Grid(grid)
    }

    fn get_mut(&mut self, position: Position) -> Option<&mut Octopus> {
        self.0.get_mut(position.y)?.get_mut(position.x)
    }

    fn flash(&mut self, position: Position) {
        let octopus = self.get_mut(position).unwrap();
        octopus.flashed = true;
        let adjacent_points = position.get_adjacent_points();
        for adjacent_point in adjacent_points {
            self.increment_energy(adjacent_point)
        }
    }

    fn increment_energy(&mut self, position: Position) {
        let octopus = match self.get_mut(position) {
            None => return,
            Some(x) => x,
        };

        if octopus.flashed {
            return;
        }

        octopus.energy += 1;
        if octopus.energy > 9 {
            self.flash(position)
        }
    }

    fn count_flashed(&self) -> usize {
        self.0.iter().flatten().filter(|x| x.flashed).count()
    }

    fn reset_flashed(&mut self) {
        for v in self.0.iter_mut() {
            for x in v.iter_mut() {
                if x.flashed {
                    x.flashed = false;
                    x.energy = 0;
                }
            }
        }
    }

    fn step(&mut self) -> usize {
        let height = self.0.len();
        let width = self.0[0].len();
        for y in 0..height {
            for x in 0..width {
                let position = Position::new(x, y);
                self.increment_energy(position);
            }
        }

        let flashed = self.count_flashed();
        self.reset_flashed();
        flashed
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let mut grid = Grid::new(&input);
    let mut grid_2 = grid.clone();
    let mut output_1 = 0;
    for _ in 0..100 {
        output_1 += grid.step()
    }

    let mut output_2 = 1;
    while grid_2.step() != 100 {
        output_2 += 1
    }
    println!("part 1: {output_1} part 2: {output_2}")
}
