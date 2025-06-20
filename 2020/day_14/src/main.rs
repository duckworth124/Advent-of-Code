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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bit {
    True,
    False,
    X,
}

struct FloatingMask([Bit;36]);

struct CompressedState {
    mask: [Bit; 36],
    memory: Vec<([Bit; 36], u64)>,
}

impl CompressedState {
    fn apply_instruction(&mut self, mut instruction: &str) {
        if let Some(s) = instruction.strip_prefix("mask = ") {
            let new_mask: [Bit; 36] = s
                .chars()
                .map(|c| match c {
                    '0' => Bit::False,
                    '1' => Bit::True,
                    'X' => Bit::X,
                    _ => panic!("unrecognized character: {c}"),
                })
                .collect::<Vec<Bit>>()
                .try_into()
                .unwrap();
            self.mask = new_mask;
            return;
        }
        let address: u64 = instruction
            .split_once('[')
            .unwrap()
            .1
            .split_once(']')
            .unwrap()
            .0
            .parse()
            .unwrap();

        let address: [Bit;36] = 

        let value: u64 = instruction.split_once("= ").unwrap().1.parse().unwrap();
    }

    fn total_value(&self) -> u64 {
        self.memory
            .iter()
            .map(|(mask, value)| size(mask) * *value)
            .sum()
    }
}

fn size(mask: &[Bit]) -> u64 {
    let xs = mask.iter().filter(|&&b| b == Bit::X).count();
    1 << xs
}

fn solve(input: &str) -> (u64, u64) {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|mut l| Instruction::parse(&mut l).unwrap())
        .collect();

    let mut state = State::default();
    for instruction in instructions {
        state.apply_instruction(instruction);
    }

    let output_1 = state.memory.values().sum();

    let mut state = CompressedState {
        mask: [Bit::False; 36],
        memory: vec![],
    };

    for line in input.lines() {
        state.apply_instruction(line);
    }

    let output_2 = state.total_value();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
