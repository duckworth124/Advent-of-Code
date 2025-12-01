use itertools::{chain, repeat_n, Itertools};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::read_to_string,
    ops::{Add, Mul, Neg, Sub},
    time::Instant,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn components(self) -> [(Direction, usize); 2] {
        let (hd, hl) = match self.x {
            ..=-1 => (Direction::Left, self.x.unsigned_abs() as usize),
            _ => (Direction::Right, self.x.unsigned_abs() as usize),
        };
        let (vd, vl) = match self.y {
            ..=-1 => (Direction::Up, self.y.unsigned_abs() as usize),
            _ => (Direction::Down, self.y.unsigned_abs() as usize),
        };
        [(hd, hl), (vd, vl)]
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

impl From<Instruction> for Position {
    fn from(value: Instruction) -> Self {
        let (x, y) = match value {
            Instruction::Press => (2, 0),
            Instruction::Move(Direction::Up) => (1, 0),
            Instruction::Move(Direction::Down) => (1, 1),
            Instruction::Move(Direction::Left) => (0, 1),
            Instruction::Move(Direction::Right) => (2, 1),
        };
        Self { x, y }
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Instruction {
    Move(Direction),
    Press,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Press => 'A',
            Self::Move(Direction::Up) => '^',
            Self::Move(Direction::Down) => 'v',
            Self::Move(Direction::Left) => '<',
            Self::Move(Direction::Right) => '>',
        };
        write!(f, "{output}")
    }
}

impl From<Direction> for Instruction {
    fn from(value: Direction) -> Self {
        Self::Move(value)
    }
}

fn expand_sequence(
    sequence: &[Instruction],
    current_depth: u32,
    max_depth: u32,
    cache: &mut HashMap<(Vec<Instruction>, u32), usize>,
) -> usize {
    let key = (sequence.to_vec(), current_depth);
    if let Some(output) = cache.get(&key) {
        return *output;
    }

    if max_depth == current_depth {
        return sequence.len();
    }
    let start_position = Position { x: 2, y: 0 };
    let positions = chain!([start_position], sequence.iter().copied().map_into()).collect_vec();
    let output = movement_sequence(&positions, current_depth + 1, max_depth, cache);
    cache.insert(key, output);
    output
}

fn move_and_press(
    from: Position,
    movement: Position,
    current_depth: u32,
    max_depth: u32,
    cache: &mut HashMap<(Vec<Instruction>, u32), usize>,
) -> usize {
    let must_avoid = if current_depth == 0 {
        Position { x: 0, y: 3 }
    } else {
        Position::default()
    };
    movement
        .components()
        .into_iter()
        .permutations(2)
        .filter(|v| from + (Position::from(v[0].0) * v[0].1 as i32) != must_avoid)
        .map(|v| {
            chain!(
                repeat_n(v[0].0, v[0].1).map_into(),
                repeat_n(v[1].0, v[1].1).map_into(),
                [Instruction::Press]
            )
            .collect_vec()
        })
        .map(|s| expand_sequence(&s, current_depth, max_depth, cache))
        .min()
        .unwrap()
}

fn movement_sequence(
    positions: &[Position],
    current_depth: u32,
    max_depth: u32,
    cache: &mut HashMap<(Vec<Instruction>, u32), usize>,
) -> usize {
    positions
        .iter()
        .copied()
        .tuple_windows()
        .map(|(x, y)| (x, y - x))
        .map(|(f, m)| move_and_press(f, m, current_depth, max_depth, cache))
        .sum()
}

fn button_position(button: char) -> Position {
    let coords = match button {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("unrecognized character: {button}"),
    };
    coords.into()
}

fn total_complexities(codes: &[&str], robot_depth: u32) -> u64 {
    let start = Position { x: 2, y: 3 };
    let mut cache = HashMap::new();
    codes
        .iter()
        .map(|code| chain!([start], code.chars().map(button_position),).collect_vec())
        .map(|positions| movement_sequence(&positions, 0, robot_depth, &mut cache))
        .zip(codes)
        .map(|(n, s)| n as u64 * s[..s.len() - 1].parse::<u64>().unwrap())
        .sum()
}

fn solve(path: &str) -> (u64, u64) {
    let input = read_to_string(path).unwrap();
    let codes = input.lines().collect_vec();
    let output_1 = total_complexities(&codes, 2);
    let output_2 = total_complexities(&codes, 25);

    (output_1, output_2)
}

fn main() {
    let now = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", now.elapsed().as_secs_f64())
}
