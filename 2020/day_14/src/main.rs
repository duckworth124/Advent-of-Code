use std::{collections::HashMap, fs::read_to_string};
use winnow::{
    ascii::dec_uint,
    combinator::{alt, delimited, preceded},
    token::rest,
    Parser, Result,
};

#[derive(Clone, Copy, Debug, Default)]
struct Mask {
    zeroes: u64,
    ones: u64,
}

impl Mask {
    const fn apply(self, value: u64) -> u64 {
        (value | self.ones) & self.zeroes
    }
}

enum Instruction {
    SetMask(Mask),
    SetMem(usize, u64),
}

impl Instruction {
    fn parse(input: &mut &str) -> Result<Self> {
        alt((Self::parse_set_mask, Self::parse_set_mem)).parse_next(input)
    }

    fn parse_set_mask(input: &mut &str) -> Result<Self> {
        preceded("mask = ", rest)
            .map(|s: &str| {
                let mut zeroes = 0;
                let mut ones = 0;
                for c in s.chars() {
                    zeroes *= 2;
                    ones *= 2;
                    if c == '1' {
                        zeroes += 1;
                        ones += 1;
                    }
                    if c == 'X' {
                        zeroes += 1;
                    }
                }
                Mask { zeroes, ones }
            })
            .map(Self::SetMask)
            .parse_next(input)
    }

    fn parse_set_mem(input: &mut &str) -> Result<Self> {
        let address: usize = delimited("mem[", dec_uint, "] = ").parse_next(input)?;
        let value: u64 = dec_uint(input)?;
        Ok(Self::SetMem(address, value))
    }
}

#[derive(Default, Debug)]
struct State {
    mask: Mask,
    memory: HashMap<usize, u64>,
}

impl State {
    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(mask) => self.mask = mask,
            Instruction::SetMem(address, value) => {
                self.memory.insert(address, self.mask.apply(value));
            }
        }
    }
}

fn solve(input: &str) -> u64 {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|mut l| Instruction::parse(&mut l).unwrap())
        .collect();

    let mut state = State::default();
    for instruction in instructions {
        state.apply_instruction(instruction);
    }

    state.memory.values().sum()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = solve(&input);
    println!("part 1: {output_1}")
}
