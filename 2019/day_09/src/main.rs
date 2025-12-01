use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

#[derive(Default, Clone)]
struct Icpu {
    data: HashMap<usize, i64>,
    pc: usize,
    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
    relative_base: i64,
}

impl Icpu {
    fn new(data: Vec<i64>) -> Self {
        let data = data.into_iter().enumerate().collect();
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

    fn get_param(&self, param_number: usize, op: i64) -> i64 {
        let mode = (op / (10_i64.pow(1 + param_number as u32))) % 10;
        let val = self.data[&(self.pc + param_number)];
        if mode == 1 {
            val
        } else if mode == 2 {
            let address = (val + self.relative_base) as usize;
            self.get(address)
        } else {
            let address = val as usize;
            self.get(address)
        }
    }

    fn write(&mut self, param_number: usize, op: i64, value: i64) {
        let mode = (op / (10_i64.pow(1 + param_number as u32))) % 10;
        let val = self.data[&(self.pc + param_number)];
        if mode == 2 {
            let address = (val + self.relative_base) as usize;
            *self.get_mut(address) = value
        } else {
            let address = val as usize;
            *self.get_mut(address) = value
        }
    }

    fn get_mut(&mut self, address: usize) -> &mut i64 {
        self.data.entry(address).or_default()
    }

    fn get(&self, address: usize) -> i64 {
        self.data.get(&address).copied().unwrap_or_default()
    }

    fn get_output(&mut self) -> Option<i64> {
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

fn solve(data: Vec<i64>) -> (i64, i64) {
    let mut icpu = Icpu::new(data.clone());
    icpu.inputs.push_back(1);
    let output_1 = icpu.get_output().unwrap();

    icpu = Icpu::new(data);
    icpu.inputs.push_back(2);
    let output_2 = icpu.get_output().unwrap();
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
