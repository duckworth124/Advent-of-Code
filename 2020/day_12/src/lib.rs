use std::{
    fmt::Debug,
    fs::read_to_string,
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction, n: i32) -> Self {
        self + (Position::from(direction) * n)
    }

    fn is_in_bounds(self, width: i32, height: i32) -> bool {
        (0..width).contains(&self.x) && (0..height).contains(&self.y)
    }

    fn manhatten_distance(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn rotate_right(mut self, n: i32) -> Self {
        for _ in 0..n {
            self = Self {
                x: -self.y,
                y: self.x,
            }
        }

        self
    }

    fn rotate_left(self, n: i32) -> Self {
        self.rotate_right(3 * n)
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        let (x, y) = match value {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        Self { x, y }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (x, y) = (self.x + rhs.x, self.y + rhs.y);
        Self { x, y }
    }
}

impl Neg for Position {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let (x, y) = (-self.x, -self.y);
        Self { x, y }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul<i32> for Position {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        let (x, y) = (self.x * rhs, self.y * rhs);
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    fn rotate_right(mut self, n: i32) -> Self {
        for _ in 0..n {
            self = match self {
                Self::Up => Self::Right,
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
            }
        }
        self
    }

    fn rotate_left(self, n: i32) -> Self {
        self.rotate_right(3 * n)
    }

    fn opposite(self) -> Self {
        self.rotate_right(2)
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct State {
    position: Position,
    direction: Direction,
}

impl State {
    fn manhatten_distance(self, other: Self) -> u32 {
        self.position.manhatten_distance(other.position)
    }
}

impl Actionable for State {
    fn apply(self, instruction: Instruction) -> Self {
        let (position, direction) = match instruction.instruction_type {
            InstructionType::Forward => (
                self.position.step(self.direction, instruction.number),
                self.direction,
            ),
            InstructionType::RotateLeft => (
                self.position,
                self.direction.rotate_left(instruction.number / 90),
            ),
            InstructionType::RotateRight => (
                self.position,
                self.direction.rotate_right(instruction.number / 90),
            ),
            InstructionType::Absolute(d) => {
                (self.position.step(d, instruction.number), self.direction)
            }
        };
        Self {
            position,
            direction,
        }
    }
}

#[derive(Clone, Copy)]
enum InstructionType {
    Absolute(Direction),
    RotateLeft,
    RotateRight,
    Forward,
}

impl InstructionType {
    fn new(c: char) -> Self {
        match c {
            'N' => Self::Absolute(Direction::Up),
            'S' => Self::Absolute(Direction::Down),
            'E' => Self::Absolute(Direction::Right),
            'W' => Self::Absolute(Direction::Left),
            'L' => Self::RotateLeft,
            'R' => Self::RotateRight,
            'F' => Self::Forward,
            _ => panic!("unrecognized character: {c}"),
        }
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    instruction_type: InstructionType,
    number: i32,
}

impl Instruction {
    fn new(input: &str) -> Self {
        let instruction_type = InstructionType::new(input.chars().next().unwrap());
        let number = input[1..].parse().unwrap();
        Self {
            instruction_type,
            number,
        }
    }
}

struct Instructions(Vec<Instruction>);

impl Instructions {
    fn new(input: &str) -> Self {
        Self(input.lines().map(Instruction::new).collect())
    }
}

#[derive(Clone, Copy, Debug)]
struct WaypointState {
    position: Position,
    waypoint_position: Position,
}

impl WaypointState {
    fn manhatten_distance(self, other: Self) -> u32 {
        self.position.manhatten_distance(other.position)
    }
}

impl Actionable for WaypointState {
    fn apply(self, instruction: Instruction) -> Self {
        let (position, waypoint_position) = match instruction.instruction_type {
            InstructionType::Absolute(d) => (
                self.position,
                self.waypoint_position.step(d, instruction.number),
            ),
            InstructionType::RotateRight => (
                self.position,
                self.waypoint_position.rotate_right(instruction.number / 90),
            ),
            InstructionType::RotateLeft => (
                self.position,
                self.waypoint_position.rotate_left(instruction.number / 90),
            ),
            InstructionType::Forward => (
                self.position + self.waypoint_position * instruction.number,
                self.waypoint_position,
            ),
        };
        Self {
            position,
            waypoint_position,
        }
    }
}

trait Actionable: Copy + Debug {
    fn apply(self, instruction: Instruction) -> Self;

    fn apply_all(self, instructions: &[Instruction]) -> Self {
        instructions.iter().fold(self, |s, i| s.apply(*i))
    }
}

pub fn solve(path: &str) -> (u32, u32) {
    let input = read_to_string(path).unwrap();
    let instructions = Instructions::new(&input);
    let start = State::default();
    let end = start.apply_all(&instructions.0);
    let output_1 = start.manhatten_distance(end);
    let start = WaypointState {
        position: Position::default(),
        waypoint_position: Position { x: 10, y: -1 },
    };
    let end = start.apply_all(&instructions.0);
    let output_2 = start.manhatten_distance(end);
    (output_1, output_2)
}
