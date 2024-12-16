use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;
use priority_queue::PriorityQueue;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::Right => (self.x + 1, self.y),
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
        };
        Self { x, y }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn rotate_left(self) -> Self {
        self.rotate_right().rotate_right().rotate_right()
    }

    fn opposite(self) -> Self {
        self.rotate_right().rotate_right()
    }
}

#[derive(Clone, Copy)]
enum Action {
    GoForward,
    TurnLeft,
    TurnRight,
}

impl Action {
    fn all() -> [Self; 3] {
        [Self::GoForward, Self::TurnLeft, Self::TurnRight]
    }

    fn cost(self) -> u32 {
        match self {
            Self::GoForward => 1,
            _ => 1000,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    position: Position,
    direction: Direction,
}

impl State {
    fn apply_action(self, action: Action) -> Self {
        let (position, direction) = match action {
            Action::GoForward => (self.position.step(self.direction), self.direction),
            Action::TurnLeft => (self.position, self.direction.rotate_left()),
            Action::TurnRight => (self.position, self.direction.rotate_right()),
        };

        Self {
            position,
            direction,
        }
    }

    fn opposite(self) -> Self {
        let direction = self.direction.opposite();
        Self { direction, ..self }
    }
}

struct Maze {
    grid: Vec<Vec<bool>>,
    start: Position,
    end: Position,
}

impl Maze {
    fn get(&self, position: Position) -> Option<bool> {
        let x: usize = position.x.try_into().ok()?;
        let y: usize = position.y.try_into().ok()?;
        self.grid.get(y)?.get(x).copied()
    }
    fn is_blocked(&self, position: Position) -> bool {
        self.get(position) != Some(false)
    }

    fn distances_from_start(&self) -> HashMap<State, u32> {
        let start_state = State {
            position: self.start,
            direction: Direction::Right,
        };
        let mut frontier: PriorityQueue<State, Reverse<u32>> =
            PriorityQueue::from(vec![(start_state, Reverse(0))]);
        let mut visited = HashSet::new();
        let mut distances = HashMap::new();
        while let Some((current_state, Reverse(current_cost))) = frontier.pop() {
            if !visited.insert(current_state) {
                continue;
            }

            if self.is_blocked(current_state.position) {
                continue;
            }

            let previous_cost = distances.entry(current_state).or_insert(current_cost);
            *previous_cost = (*previous_cost).min(current_cost);

            for action in Action::all() {
                let new_state = current_state.apply_action(action);
                let cost = action.cost();
                let new_cost = current_cost + cost;
                frontier.push_increase(new_state, Reverse(new_cost));
            }
        }

        distances
    }

    fn distances_to_end(&self) -> HashMap<State, u32> {
        let start_states = [
            State {
                position: self.end,
                direction: Direction::Down,
            },
            State {
                position: self.end,
                direction: Direction::Left,
            },
        ];
        let mut frontier: PriorityQueue<State, Reverse<u32>> =
            start_states.into_iter().map(|s| (s, Reverse(0))).collect();
        let mut visited = HashSet::new();
        let mut distances = HashMap::new();
        while let Some((current_state, Reverse(current_cost))) = frontier.pop() {
            if !visited.insert(current_state) {
                continue;
            }

            if self.is_blocked(current_state.position) {
                continue;
            }

            let previous_cost = distances
                .entry(current_state.opposite())
                .or_insert(current_cost);
            *previous_cost = (*previous_cost).min(current_cost);

            for action in Action::all() {
                let new_state = current_state.apply_action(action);
                let cost = action.cost();
                let new_cost = current_cost + cost;
                frontier.push_increase(new_state, Reverse(new_cost));
            }
        }

        distances
    }

    fn shortest_paths_through_here(&self) -> HashMap<State, u32> {
        let distances_from_start = self.distances_from_start();
        let distances_to_end = self.distances_to_end();
        distances_from_start
            .keys()
            .map(|s| (*s, distances_from_start.get(s), distances_to_end.get(s)))
            .flat_map(|(s, x, y)| Some((s, x?, y?)))
            .map(|(s, x, y)| (s, *x + *y))
            .collect()
    }
}

fn solve(path: &str) -> (u32, usize) {
    let input = read_to_string(path).unwrap();
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();
    let start = Position {
        x: 1,
        y: (grid.len() - 2) as i32,
    };
    let end = Position {
        y: 1,
        x: (grid[0].len() - 2) as i32,
    };
    let maze = Maze { grid, start, end };
    let start_state = State {
        position: maze.start,
        direction: Direction::Right,
    };
    let output_1 = maze.distances_to_end()[&start_state];
    let output_2 = maze
        .shortest_paths_through_here()
        .into_iter()
        .filter(|(_, d)| *d == output_1)
        .map(|(s, _)| s.position)
        .unique()
        .count();

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
