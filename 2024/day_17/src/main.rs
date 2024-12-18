use regex::Regex;
use std::{fs::read_to_string, time::Instant};

#[derive(Clone, Copy)]
struct State {
    register_a: u64,
    register_b: u64,
    register_c: u64,
}

struct Cpu {
    state: State,
    instructions: Vec<u64>,
    program_counter: usize,
}

impl Cpu {
    fn get_instruction(&self) -> Option<Instruction> {
        let opcode = *self.instructions.get(self.program_counter)?;
        let instruction = match opcode {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => return None,
        };

        Some(instruction)
    }

    fn get_operand(&self, instruction: Instruction) -> Option<u64> {
        let operand = *self.instructions.get(self.program_counter + 1)?;
        let operand_type = instruction.operand_type();
        let output = match operand_type {
            OperandType::Literal => operand,
            OperandType::Combo => match operand {
                0..=3 => operand,
                4 => self.state.register_a,
                5 => self.state.register_b,
                6 => self.state.register_c,
                7 => panic!("reserved operand"),
                _ => panic!("unexpected_operand: {operand}"),
            },
        };

        Some(output)
    }

    fn step(&mut self) -> (Option<u64>, bool) {
        let instruction = match self.get_instruction() {
            Some(i) => i,
            None => return (None, false),
        };

        let operand = match self.get_operand(instruction) {
            Some(o) => o,
            None => return (None, false),
        };

        let mut output = None;

        match instruction {
            Instruction::Adv => self.state.register_a /= 2_u64.pow(operand as u32),
            Instruction::Bxl => self.state.register_b ^= operand,
            Instruction::Bst => self.state.register_b = operand % 8,
            Instruction::Jnz => {
                if self.state.register_a != 0 {
                    self.program_counter = operand as usize
                } else {
                    self.program_counter += 2
                }
            }
            Instruction::Bxc => self.state.register_b ^= self.state.register_c,
            Instruction::Out => output = Some(operand % 8),
            Instruction::Bdv => {
                self.state.register_b = self.state.register_a / 2_u64.pow(operand as u32)
            }
            Instruction::Cdv => {
                self.state.register_c = self.state.register_a / 2_u64.pow(operand as u32)
            }
        };

        if instruction != Instruction::Jnz {
            self.program_counter += 2
        }

        (output, true)
    }

    fn run(&mut self) -> Vec<u64> {
        let mut output = vec![];
        loop {
            let (out, run) = self.step();

            output.push(out);
            if !run {
                break;
            }
        }

        output.into_iter().flatten().collect()
    }

    fn next_output(&mut self) -> Option<u64> {
        loop {
            let (out, run) = self.step();
            if !run {
                return None;
            }

            if out.is_some() {
                return out;
            }
        }
    }

    fn find_a(&self, target: &[u64], current: u64) -> Vec<u64> {
        let last = match target.last() {
            Some(x) => *x,
            None => return vec![current],
        };

        (0..8)
            .map(|b| current * 8 + b)
            .filter(|&a| {
                let new_state = State {
                    register_a: a,
                    register_b: 0,
                    register_c: 0,
                };
                let mut new_cpu = Cpu {
                    state: new_state,
                    instructions: self.instructions.clone(),
                    program_counter: 0,
                };

                let out = new_cpu.next_output();
                out == Some(last)
            })
            .flat_map(|a| self.find_a(&target[..target.len() - 1], a))
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn operand_type(self) -> OperandType {
        match self {
            Self::Adv | Self::Bst | Self::Out | Self::Bdv | Self::Cdv => OperandType::Combo,
            _ => OperandType::Literal,
        }
    }
}

enum OperandType {
    Literal,
    Combo,
}

fn all_numbers(input: &str) -> Vec<u64> {
    let pat = Regex::new(r"\d+").unwrap();
    pat.find_iter(input)
        .map(|s| s.as_str().parse().unwrap())
        .collect()
}

fn solve(path: &str) -> (String, u64) {
    let input = read_to_string(path).unwrap();
    let nums = all_numbers(&input);
    let (register_a, register_b, register_c) = (nums[0], nums[1], nums[2]);
    let instructions = nums[3..].to_vec();
    let state = State {
        register_a,
        register_b,
        register_c,
    };
    let mut cpu = Cpu {
        state,
        instructions,
        program_counter: 0,
    };

    let output_1 = cpu
        .run()
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let output_2 = cpu.find_a(&cpu.instructions, 0)[0];
    let state = State {
        register_a: output_2,
        ..state
    };
    let mut cpu = Cpu {
        state,
        program_counter: 0,
        ..cpu
    };
    debug_assert_eq!(cpu.run(), cpu.instructions);
    (output_1, output_2)
}

fn main() {
    let now = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", now.elapsed().as_secs_f64())
}
