use std::fs::read_to_string;

#[derive(Default)]
struct Icpu {
    data: Vec<i32>,
    pc: usize,
    inputs: Vec<i32>,
    outputs: Vec<i32>,
}

impl Icpu {
    fn new(data: Vec<i32>) -> Self {
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
                let out = self.data[self.pc + 3] as usize;
                self.data[out] = par1 + par2;
                self.pc += 4;
            }
            2 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                let out = self.data[self.pc + 3] as usize;
                self.data[out] = par1 * par2;
                self.pc += 4;
            }
            3 => {
                let out = self.data[self.pc + 1] as usize;
                self.data[out] = self.inputs.pop().unwrap();
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

                let out = self.data[self.pc + 3] as usize;
                if par1 < par2 {
                    self.data[out] = 1;
                } else {
                    self.data[out] = 0;
                }
                self.pc += 4;
            }

            8 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                let out = self.data[self.pc + 3] as usize;
                if par1 == par2 {
                    self.data[out] = 1;
                } else {
                    self.data[out] = 0
                }
                self.pc += 4;
            }
            99 => return false,

            _ => {
                panic!("unrecoginized opcode: {op}")
            }
        };

        true
    }

    fn get_param(&self, param_number: usize, op: i32) -> i32 {
        let mode = (op / (10_i32.pow(1 + param_number as u32))) % 10;
        let val = self.data[self.pc + param_number];
        if mode == 1 {
            val
        } else {
            self.data[val as usize]
        }
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

fn solve(data: Vec<i32>) -> (i32, i32) {
    let mut icpu = Icpu::new(data.clone());
    icpu.inputs.push(1);
    icpu.run();
    let output_1 = *icpu.outputs.last().unwrap();
    icpu = Icpu::new(data);
    icpu.inputs.push(5);
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
