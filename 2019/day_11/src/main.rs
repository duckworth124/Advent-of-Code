use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs::read_to_string,
};

use itertools::Itertools;

#[derive(Default, Clone)]
struct Icpu {
    data: HashMap<i128, i128>,
    pc: i128,
    inputs: VecDeque<i128>,
    outputs: Vec<i128>,
    relative_base: i128,
}

impl Icpu {
    fn new(data: Vec<i128>) -> Self {
        let data = data
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i as i128, x))
            .collect();
        Self {
            data,
            ..Default::default()
        }
    }

    fn step(&mut self) -> bool {
        let op = self.get(self.pc);
        match op % 100 {
            1 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                self.write(3, op, par1 + par2);
                self.pc += 4;
            }

            2 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                self.write(3, op, par1 * par2);
                self.pc += 4;
            }

            3 => {
                let input = self.inputs.pop_front().unwrap();
                self.write(1, op, input);
                self.pc += 2;
            }

            4 => {
                let par = self.get_param(1, op);
                self.outputs.push(par);
                self.pc += 2;
            }

            5 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                if par1 != 0 {
                    self.pc = par2;
                } else {
                    self.pc += 3
                }
            }

            6 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                if par1 == 0 {
                    self.pc = par2;
                } else {
                    self.pc += 3
                }
            }

            7 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);

                let output = (par1 < par2) as i128;
                self.write(3, op, output);
                self.pc += 4;
            }

            8 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                let output = (par1 == par2) as i128;
                self.write(3, op, output);
                self.pc += 4;
            }

            9 => {
                let par = self.get_param(1, op);
                self.relative_base += par;
                self.pc += 2;
            }

            99 => return false,

            _ => {
                panic!("unrecoginized opcode: {op}")
            }
        };

        true
    }

    fn get_param(&self, param_number: i128, op: i128) -> i128 {
        let mode = (op / (10_i128.pow(1 + param_number as u32))) % 10;
        let val = self.data[&(self.pc + param_number)];
        if mode == 1 {
            val
        } else if mode == 2 {
            let address = val + self.relative_base;
            self.get(address)
        } else {
            let address = val;
            self.get(address)
        }
    }

    fn write(&mut self, param_number: i128, op: i128, value: i128) {
        let mode = (op / (10_i128.pow(1 + param_number as u32))) % 10;
        let val = self.data[&(self.pc + param_number)];
        if mode == 2 {
            let address = val + self.relative_base;
            *self.get_mut(address) = value
        } else {
            let address = val;
            *self.get_mut(address) = value
        }
    }

    fn get_mut(&mut self, address: i128) -> &mut i128 {
        self.data.entry(address).or_default()
    }

    fn get(&self, address: i128) -> i128 {
        self.data.get(&address).copied().unwrap_or_default()
    }

    fn get_output(&mut self) -> Option<i128> {
        loop {
            let output = self.outputs.pop();
            if output.is_some() {
                return output;
            }
            if !self.step() {
                return None;
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    const fn rotate_left(self) -> Self {
        self.rotate_right().rotate_right().rotate_right()
    }

    const fn to_coords(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

struct State {
    position: (i32, i32),
    direction: Direction,
    white_tiles: HashSet<(i32, i32)>,
    painted_tiles: HashSet<(i32, i32)>,
    cpu: Icpu,
}

impl State {
    fn step(&mut self) -> bool {
        let is_white = self.white_tiles.contains(&self.position);
        self.cpu.inputs.push_back(is_white as i128);
        match self.cpu.get_output() {
            Some(0) => {
                self.white_tiles.remove(&self.position);
            }
            Some(1) => {
                self.white_tiles.insert(self.position);
            }
            None => return false,
            x => panic!("unrecognized output: {x:?}"),
        }
        self.painted_tiles.insert(self.position);
        match self.cpu.get_output() {
            Some(0) => self.direction = self.direction.rotate_left(),
            Some(1) => self.direction = self.direction.rotate_right(),
            None => return false,
            x => panic!("unrecognized output: {x:?}"),
        }
        assert!(self.cpu.inputs.is_empty());
        let (dx, dy) = self.direction.to_coords();
        self.position.0 += dx;
        self.position.1 += dy;

        true
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_y, max_y) = self
            .painted_tiles
            .iter()
            .map(|p| p.1)
            .minmax()
            .into_option()
            .unwrap_or_default();

        let (min_x, max_x) = self
            .painted_tiles
            .iter()
            .map(|p| p.0)
            .minmax()
            .into_option()
            .unwrap_or_default();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = (x, y);
                if self.white_tiles.contains(&p) {
                    write!(f, "#")?;
                    continue;
                }
                write!(f, "'")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve(input: &str) -> (usize, String) {
    let data: Vec<i128> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut state = State {
        position: (0, 0),
        white_tiles: HashSet::new(),
        painted_tiles: HashSet::new(),
        direction: Direction::Up,
        cpu: Icpu::new(data.clone()),
    };
    while state.step() {}
    let output_1 = state.painted_tiles.len();
    state = State {
        position: (0, 0),
        white_tiles: HashSet::new(),
        painted_tiles: HashSet::new(),
        direction: Direction::Up,
        cpu: Icpu::new(data),
    };
    state.white_tiles.insert((0, 0));
    while state.step() {}
    let output_2 = state.to_string();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2:\n{output_2}")
}
