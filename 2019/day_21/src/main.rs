use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;

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

fn solve(input: &str) -> (i128, i128) {
    let data: Vec<i128> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut cpu = Icpu::new(data.clone());
    cpu.inputs.extend(
        "NOT T T
AND A T
AND B T
AND C T
NOT T T
NOT D J
NOT J J
AND T J
WALK
"
        .chars()
        .map(|c| c as u8 as i128),
    );
    cpu.run();
    let display = cpu
        .outputs
        .iter()
        .map(|&x| x as u8 as char)
        .collect::<String>();
    println!("{display}");
    let output_1 = *cpu.outputs.last().unwrap();

    let mut cpu = Icpu::new(data);
    cpu.inputs.extend(
        "OR D J
OR E T
OR H T
AND T J
NOT A T
NOT T T
AND B T
AND C T
NOT T T
AND T J
RUN
"
        .chars()
        .map(|c| c as u8 as i128),
    );
    cpu.run();
    let display = cpu
        .outputs
        .iter()
        .map(|&x| x as u8 as char)
        .collect::<String>();
    println!("{display}");
    let output_2 = *cpu.outputs.last().unwrap();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}")
}
