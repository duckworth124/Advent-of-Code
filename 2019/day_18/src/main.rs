use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    fs::read_to_string,
    hash::Hash,
};

type Position = (usize, usize);

#[derive(Clone, Copy, Eq)]
struct State<'a> {
    map: &'a HashMap<Position, HashMap<Position, u32>>,
    grid: &'a [Vec<Tile>],
    position: Position,
    keys: [bool; 26],
}

impl State<'_> {
    fn min_steps(self) -> u32 {
        let mut frontier: PriorityQueue<Self, Reverse<u32>> =
            PriorityQueue::from_iter([(self, Reverse(0))]);
        let mut seen = HashSet::new();
        loop {
            let (current, Reverse(current_cost)) = frontier.pop().unwrap();
            if !seen.insert(current) {
                continue;
            }
            if current.is_goal() {
                return current_cost;
            }
            for (next, cost) in current
                .possible_next()
                .into_iter()
                .filter_map(|(p, c)| Some((current.step(p)?, c)))
            {
                frontier.push_increase(next, Reverse(cost + current_cost));
            }
        }
    }

    fn is_goal(self) -> bool {
        self.keys.into_iter().all(|b| b)
    }

    fn possible_next(self) -> Vec<(Position, u32)> {
        self.map
            .get(&self.position)
            .map(|m| m.iter().map(|(l, r)| (*l, *r)).collect())
            .unwrap_or_default()
    }

    fn step(mut self, new_position: Position) -> Option<Self> {
        let (x, y) = new_position;
        let new_tile = self.grid[y][x];
        match new_tile {
            Tile::Start => {}
            Tile::Key(n) => self.keys[n] = true,
            Tile::Lock(n) => {
                if !self.keys[n] {
                    return None;
                }
            }
            _ => unreachable!(),
        }
        self.position = new_position;
        Some(self)
    }
}

impl Hash for State<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.keys.hash(state);
    }
}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.keys == other.keys
    }
}

impl Debug for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "position: {:?}", self.position)?;
        write!(f, "keys: ")?;
        for b in self.keys {
            let c = if b { 'O' } else { '#' };
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Eq)]
struct StateWithRobots<'a> {
    positions: [Position; 4],
    grid: &'a [Vec<Tile>],
    map: &'a HashMap<Position, HashMap<Position, u32>>,
    keys: [bool; 26],
}

impl StateWithRobots<'_> {
    fn is_goal(self) -> bool {
        self.keys.into_iter().all(|b| b)
    }

    fn possible_next(self) -> Vec<(usize, Position, u32)> {
        self.positions
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| {
                self.map
                    .get(&p)
                    .map(|m| m.iter().map(move |(&p2, &d)| (i, p2, d)))
            })
            .flatten()
            .collect()
    }

    fn step(mut self, robot_index: usize, next_position: Position) -> Option<Self> {
        let (x, y) = next_position;
        let tile = self.grid[y][x];
        match tile {
            Tile::Start => {}
            Tile::Key(n) => self.keys[n] = true,
            Tile::Lock(n) => {
                if !self.keys[n] {
                    return None;
                }
            }
            _ => unreachable!(),
        }
        self.positions[robot_index] = next_position;
        Some(self)
    }

    fn min_steps(self) -> u32 {
        let mut frontier: PriorityQueue<Self, Reverse<u32>> =
            PriorityQueue::from_iter([(self, Reverse(0))]);
        let mut seen = HashSet::new();
        loop {
            let (current, Reverse(current_cost)) = frontier.pop().unwrap();
            if !seen.insert(current) {
                continue;
            }
            if current.is_goal() {
                return current_cost;
            }
            for (next, cost) in current
                .possible_next()
                .into_iter()
                .filter_map(|(i, p, c)| Some((current.step(i, p)?, c)))
            {
                frontier.push_increase(next, Reverse(cost + current_cost));
            }
        }
    }
}

impl Hash for StateWithRobots<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.positions.hash(state);
        self.keys.hash(state);
    }
}

impl PartialEq for StateWithRobots<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions && self.keys == other.keys
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Tile {
    Open,
    Wall,
    Start,
    Key(usize),
    Lock(usize),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(self, position: Position) -> Option<Position> {
        let (x, y) = position;
        let (new_x, new_y) = match self {
            Self::Up => (x, y.checked_sub(1)?),
            Self::Down => (x, y + 1),
            Self::Left => (x.checked_sub(1)?, y),
            Self::Right => (x + 1, y),
        };
        Some((new_x, new_y))
    }
}

fn generate_map(
    grid: &[Vec<Tile>],
    starts: Vec<Position>,
) -> HashMap<Position, HashMap<Position, u32>> {
    let width = grid[0].len();
    let height = grid.len();
    let mut map: HashMap<Position, HashMap<Position, u32>> = HashMap::new();
    let mut visited_points_of_interest = HashSet::new();
    let mut points_of_interest = starts;
    while let Some(poi) = points_of_interest.pop() {
        if !visited_points_of_interest.insert(poi) {
            continue;
        }
        let mut frontier = VecDeque::from([(poi, 0)]);
        let mut seen = HashSet::new();
        while let Some((current, cost)) = frontier.pop_front() {
            if !seen.insert(current) {
                continue;
            }
            let (x, y) = current;
            let tile = grid[y][x];
            match tile {
                Tile::Open => {}
                Tile::Wall => {
                    continue;
                }
                _ if current != poi => {
                    points_of_interest.push(current);
                    map.entry(current).or_default().insert(poi, cost);
                    map.entry(poi).or_default().insert(current, cost);
                    continue;
                }
                _ => {}
            }
            frontier.extend(
                [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]
                .into_iter()
                .filter_map(|d| d.step(current))
                .filter(|&(x, y)| x < width && y < height)
                .map(|p| (p, cost + 1)),
            );
        }
    }
    map
}

fn update_grid(grid: &mut [Vec<Tile>], start: Position) {
    let (x, y) = start;
    for (x, y) in [
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ] {
        grid[y][x] = Tile::Start
    }
    for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        grid[y][x] = Tile::Wall
    }
    grid[y][x] = Tile::Wall;
}

fn solve(input: &str) -> (u32, u32) {
    let mut start = None;
    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    '@' => {
                        start = Some((x, y));
                        Tile::Start
                    }
                    'a'..='z' => Tile::Key(c as usize - 'a' as usize),
                    'A'..='Z' => Tile::Lock(c as usize - 'A' as usize),
                    _ => panic!("unreachable char: {c}"),
                })
                .collect()
        })
        .collect();
    let start = start.unwrap();
    let map = generate_map(&grid, vec![start]);
    let initial_state = State {
        map: &map,
        grid: &grid,
        position: start,
        keys: [false; 26],
    };
    let output_1 = initial_state.min_steps();
    update_grid(&mut grid, start);
    let (x, y) = start;
    let starts = [
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ];
    let map = generate_map(&grid, starts.to_vec());
    let initial_state = StateWithRobots {
        positions: starts,
        grid: &grid,
        keys: [false; 26],
        map: &map,
    };
    let output_2 = initial_state.min_steps();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
