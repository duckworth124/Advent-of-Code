use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    iter,
    ops::Add,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn manhatten_distance(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn surrounding(self) -> [Self; 4] {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .map(|(x, y)| Self { x, y })
            .map(|p| p + self)
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (x, y) = (self.x + rhs.x, self.y + rhs.y);
        Self { x, y }
    }
}

struct Maze {
    grid: Vec<Vec<bool>>,
    start: Position,
}

impl Maze {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        let start = input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.char_indices().map(move |(x, c)| (x, y, c)))
            .map(|(x, y, c)| (x as i32, y as i32, c))
            .map(|(x, y, c)| (Position { x, y }, c))
            .find(|(_, c)| *c == 'S')
            .unwrap()
            .0;

        Self { grid, start }
    }

    fn get(&self, position: Position) -> Option<bool> {
        let y: usize = position.y.try_into().ok()?;
        let x: usize = position.x.try_into().ok()?;
        self.grid.get(y)?.get(x).copied()
    }

    fn is_blocked(&self, position: Position) -> bool {
        self.get(position) != Some(false)
    }

    fn distances_from_start(&self) -> HashMap<Position, usize> {
        let mut visited = HashSet::new();
        iter::successors(Some(self.start), |p| {
            visited.insert(*p);
            p.surrounding()
                .into_iter()
                .filter(|p| !self.is_blocked(*p))
                .find(|p| !visited.contains(p))
        })
        .enumerate()
        .map(|(d, p)| (p, d))
        .collect()
    }
}

fn count_valid_cheats_from(
    distances: &HashMap<Position, usize>,
    from: Position,
    time: u32,
) -> usize {
    distances
        .iter()
        .map(|(p, d)| (p, d, p.manhatten_distance(from)))
        .filter(|(_, _, d)| *d <= time)
        .filter(|&(_, d, m)| *d >= distances[&from] + 100 + m as usize)
        .count()
}

fn count_all_valid_cheats(distances: &HashMap<Position, usize>, time: u32) -> usize {
    distances
        .keys()
        .map(|&from| count_valid_cheats_from(distances, from, time))
        .sum()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let maze = Maze::new(&input);
    let distances = maze.distances_from_start();
    let output_1 = count_all_valid_cheats(&distances, 2);
    let output_2 = count_all_valid_cheats(&distances, 20);

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
