use std::{collections::VecDeque, fs::read_to_string};

#[derive(Default, Clone)]
struct Icpu {
    data: Vec<i64>,
    pc: usize,
    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
    relative_base: i64,
}

impl Icpu {
    fn new(data: Vec<i64>) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    fn step(&mut self) -> bool {
        let op = self.data[self.pc];
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
                self.write(3, op, input);
                self.pc += 2;
            }

            4 => {
                let par = self.get_param(1, op);
                self.outputs.push(par);
                self.pc += 2;
            }

            5 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op) as usize;
                if par1 != 0 {
                    self.pc = par2;
                } else {
                    self.pc += 3
                }
            }

            6 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op) as usize;
                if par1 == 0 {
                    self.pc = par2;
                } else {
                    self.pc += 3
                }
            }

            7 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);

                let output = (par1 < par2) as i64;
                self.write(3, op, output);
                self.pc += 4;
            }

            8 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                let output = (par1 == par2) as i64;
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

    fn get_param(&mut self, param_number: usize, op: i64) -> i64 {
        let mode = (op / (10_i64.pow(1 + param_number as u32))) % 10;
        let val = self.data[self.pc + param_number];
        if mode == 1 {
            val
        } else if mode == 2 {
            let address = (val + self.relative_base) as usize;
            *self.get_or_extend(address)
        } else {
            let address = val as usize;
            *self.get_or_extend(address)
        }
    }

    fn write(&mut self, param_number: usize, op: i64, value: i64) {
        let mode = (op / (10_i64.pow(1 + param_number as u32))) % 10;
        let val = self.data[self.pc + param_number];
        if mode == 2 {
            let address = (val + self.relative_base) as usize;
            *self.get_or_extend(address) = value
        } else {
            let address = val as usize;
            *self.get_or_extend(address) = value
        }
    }

    fn get_or_extend(&mut self, address: usize) -> &mut i64 {
        if address >= self.data.len() {
            self.data.resize(address + 1, 0);
        }

        self.data.get_mut(address).unwrap()
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

fn solve(data: Vec<i64>) -> (i64, i64) {
    let mut icpu = Icpu::new(data.clone());
    icpu.inputs.push_back(1);
    icpu.run();
    let output_1 = *icpu.outputs.last().unwrap();

    icpu = Icpu::new(data);
    icpu.inputs.push_back(2);
    icpu.run();
    let output_2 = *icpu.outputs.last().unwrap();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let data = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let (output_1, output_2) = solve(data);
    println!("part 1: {output_1} part 2: {output_2}")
}
