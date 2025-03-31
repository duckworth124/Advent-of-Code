use itertools::Itertools;
use std::{collections::VecDeque, fs::read_to_string};

#[derive(Default, Clone)]
struct Icpu {
    data: Vec<i32>,
    pc: usize,
    inputs: VecDeque<i32>,
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
                let out = self.get_param(1, op) as usize;
                self.data[out] = self.inputs.pop_front().unwrap();
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

    fn run_till_output(&mut self) -> Option<i32> {
        let len = self.outputs.len();
        while self.step() {
            if self.outputs.len() > len {
                return self.outputs.last().copied();
            }
        }

        None
    }
}

fn simulate_amplifier(phase_setting: i32, signal: i32, icpu_data: Vec<i32>) -> i32 {
    let mut icpu = Icpu::new(icpu_data);
    icpu.inputs.extend([phase_setting, signal]);
    icpu.run();
    *icpu.outputs.last().unwrap()
}

fn simulate_all_amplifiers(phase_settings: &[i32], icpu_data: Vec<i32>) -> i32 {
    let mut signal = 0;
    for &setting in phase_settings {
        signal = simulate_amplifier(setting, signal, icpu_data.clone())
    }

    signal
}

fn simulate_feedback_loop(phase_settings: &[i32], icpu_data: Vec<i32>) -> i32 {
    let mut cpus = vec![Icpu::new(icpu_data); phase_settings.len()];
    for (cpu, setting) in cpus.iter_mut().zip(phase_settings) {
        cpu.inputs.push_back(*setting);
    }
    let mut signal = 0;
    let mut i = 0;
    loop {
        cpus[i].inputs.push_back(signal);
        let output = cpus[i].run_till_output();
        signal = match output {
            Some(x) => x,
            None => break,
        };
        i += 1;
        i %= cpus.len();
    }

    cpus.last_mut().unwrap().run();
    *cpus.last().unwrap().outputs.last().unwrap()
}

fn solve(data: Vec<i32>) -> (i32, i32) {
    let output_1 = (0..5)
        .permutations(5)
        .map(|v| simulate_all_amplifiers(&v, data.clone()))
        .max()
        .unwrap();

    let output_2 = (5..10)
        .permutations(5)
        .map(|v| simulate_feedback_loop(&v, data.clone()))
        .max()
        .unwrap();

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
