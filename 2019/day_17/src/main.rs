use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

use itertools::iproduct;

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

    fn run(&mut self) {
        while self.step() {}
    }
}

struct State {
    grid: Vec<Vec<bool>>,
}

impl State {
    fn alignment_parameters(&self) -> usize {
        let width = self.grid[0].len();
        let height = self.grid.len();
        iproduct!(1..width - 1, 1..height - 1)
            .filter(|&(x, y)| {
                [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .into_iter()
                    .all(|(x, y)| self.grid[y][x])
            })
            .map(|(x, y)| x * y)
            .sum()
    }
}

fn solve(input: &str) -> (usize, i128) {
    let data: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut cpu = Icpu::new(data.clone());
    cpu.run();
    let s: String = cpu
        .outputs
        .iter()
        .copied()
        .map(|x| x as u8 as char)
        .collect();

    let grid = s
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c != '.').collect())
        .collect();
    let state = State { grid };
    cpu = Icpu::new(data);
    cpu.data.insert(0, 2);

    let solution = "A,B,A,C,A,B,C,B,C,B
R,10,R,10,R,6,R,4
R,10,R,10,L,4
R,4,L,4,L,10,L,10
n
";

    cpu.inputs.extend(solution.chars().map(|c| c as i128));
    cpu.run();
    let output_1 = state.alignment_parameters();
    let output_2 = *cpu.outputs.last().unwrap();
    let s: String = cpu.outputs.iter().map(|x| *x as u8 as char).collect();
    println!("{s}");
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
