use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_left(self) -> Self {
        self.rotate_right().rotate_right().rotate_right()
    }
}

#[derive(Clone, Copy, Debug)]
struct Agent {
    position: Position,
    facing: Direction,
}

impl Agent {
    fn step(&mut self) {
        self.position = self.position.step(self.facing)
    }
}
struct State {
    agent: Agent,
    grid: HashSet<Position>,
}

impl State {
    fn update(&mut self) -> bool {
        if self.grid.contains(&self.agent.position) {
            self.agent.facing = self.agent.facing.rotate_right();
            self.grid.remove(&self.agent.position);
            self.agent.step();
            false
        } else {
            self.agent.facing = self.agent.facing.rotate_left();
            self.grid.insert(self.agent.position);
            self.agent.step();
            true
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
enum Status {
    #[default]
    Clean,
    Weakened,
    Infected,
    Flagged,
}

struct EvolvedState {
    agent: Agent,
    grid: HashMap<Position, Status>,
}

impl EvolvedState {
    fn update(&mut self) -> bool {
        let status = self
            .grid
            .get(&self.agent.position)
            .copied()
            .unwrap_or_default();

        match status {
            Status::Clean => {
                self.grid.insert(self.agent.position, Status::Weakened);
                self.agent.facing = self.agent.facing.rotate_left();
                self.agent.step();
                false
            }
            Status::Weakened => {
                self.grid.insert(self.agent.position, Status::Infected);
                self.agent.step();
                true
            }
            Status::Infected => {
                self.grid.insert(self.agent.position, Status::Flagged);
                self.agent.facing = self.agent.facing.rotate_right();
                self.agent.step();
                false
            }
            Status::Flagged => {
                self.grid.insert(self.agent.position, Status::Clean);
                self.agent.facing = self.agent.facing.rotate_left().rotate_left();
                self.agent.step();
                false
            }
        }
    }
}

fn solve(input: &str) -> (u32, u32) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid: HashSet<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.char_indices().map(move |(x, c)| (x, y, c)))
        .filter(|&(_, _, c)| c == '#')
        .map(|(x, y, _)| Position {
            x: x as i32,
            y: y as i32,
        })
        .collect();
    let position = Position {
        x: width as i32 / 2,
        y: height as i32 / 2,
    };
    let agent = Agent {
        position,
        facing: Direction::Up,
    };

    let mut output_1 = 0;
    let mut state = State {
        agent,
        grid: grid.clone(),
    };
    for _ in 0..10_000 {
        if state.update() {
            output_1 += 1;
        }
    }

    let grid: HashMap<Position, Status> = grid.into_iter().map(|p| (p, Status::Infected)).collect();
    let mut evolved_state = EvolvedState { grid, agent };
    let mut output_2 = 0;
    for _ in 0..10_000_000 {
        if evolved_state.update() {
            output_2 += 1
        }
    }

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
