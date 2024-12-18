use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashSet, fmt::Display, fs::read_to_string, time::Instant};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let (x, y) = (self.x + dx, self.y + dy);
        Self { x, y }
    }

    fn mahnatten_distance(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Node {
    position: Position,
    cost: u32,
}

struct Maze {
    grid: Vec<Vec<bool>>,
}

impl Maze {
    fn new(coordinates: &[(usize, usize)], dimensions: usize) -> Self {
        let mut grid = vec![vec![false; dimensions]; dimensions];
        for &(x, y) in coordinates {
            grid[y][x] = true
        }

        Self { grid }
    }

    fn is_blocked(&self, position: Position) -> bool {
        self.get(position) != Some(false)
    }

    fn get(&self, position: Position) -> Option<bool> {
        let x: usize = position.x.try_into().ok()?;
        let y: usize = position.y.try_into().ok()?;
        self.grid.get(y)?.get(x).copied()
    }

    fn shortest_path(&self) -> Option<u32> {
        let width = self.grid[0].len() as i32;
        let height = self.grid.len() as i32;
        let goal = Position {
            x: width - 1,
            y: height - 1,
        };
        let start_node = Node::default();
        let mut visited = HashSet::new();
        let mut frontier: PriorityQueue<Node, Reverse<u32>> =
            PriorityQueue::from(vec![(start_node, Reverse(0))]);
        while let Some((current_node, _)) = frontier.pop() {
            if !visited.insert(current_node.position) {
                continue;
            }

            if self.is_blocked(current_node.position) {
                continue;
            }

            if current_node.position == goal {
                return Some(current_node.cost);
            }

            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let next_position = current_node.position.step(direction);
                let next_cost = current_node.cost + 1;
                let heuristic = next_position.mahnatten_distance(goal);
                let next_node = Node {
                    position: next_position,
                    cost: next_cost,
                };
                frontier.push(next_node, Reverse(next_cost + heuristic));
            }
        }

        None
    }

    fn first_coords_that_disconnect(&mut self, coordinates: &[(usize, usize)]) -> (usize, usize) {
        let (mut l, mut r) = (0, coordinates.len() - 1);
        while r > l + 1 {
            let m = (l + r) / 2;
            *self = Maze::new(&coordinates[..=m], self.grid.len());
            if self.shortest_path().is_none() {
                r = m;
            } else {
                l = m
            }
        }

        coordinates[r]
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for tile in row {
                if *tile {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f)?
        }

        Ok(())
    }
}

fn solve(path: &str) -> (u32, (usize, usize)) {
    let input = read_to_string(path).unwrap();
    let coordinates = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let num_coords = if path == "input" { 1024 } else { 12 };
    let grid_dimensions = if path == "input" { 71 } else { 7 };
    let maze = Maze::new(&coordinates[..num_coords], grid_dimensions);
    let output_1 = maze.shortest_path().unwrap();

    let mut maze = Maze::new(&[], grid_dimensions);
    let output_2 = maze.first_coords_that_disconnect(&coordinates);

    (output_1, output_2)
}

fn main() {
    let now = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2:?}");
    println!("time: {}s", now.elapsed().as_secs_f64())
}
