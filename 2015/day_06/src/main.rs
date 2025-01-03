use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u32},
    combinator::value,
    multi::separated_list0,
    sequence::separated_pair,
    IResult, Parser,
};
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Range2D {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

impl Range2D {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            separated_pair(u32, tag(","), u32),
            tag(" through "),
            separated_pair(u32, tag(","), u32),
        )
        .map(|((a, b), (c, d))| (a as usize, b as usize, c as usize, d as usize))
        .map(|(min_x, min_y, max_x, max_y)| Self {
            min_x,
            min_y,
            max_x,
            max_y,
        })
        .parse(input)
    }
}

#[derive(Clone, Copy)]
enum InstructionType {
    On,
    Off,
    Toggle,
}

impl InstructionType {
    fn parse_on(input: &str) -> IResult<&str, Self> {
        value(Self::On, tag("turn on")).parse(input)
    }

    fn parse_off(input: &str) -> IResult<&str, Self> {
        value(Self::Off, tag("turn off")).parse(input)
    }

    fn parse_toggle(input: &str) -> IResult<&str, Self> {
        value(Self::Toggle, tag("toggle")).parse(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_on, Self::parse_off, Self::parse_toggle)).parse(input)
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    instruction_type: InstructionType,
    range: Range2D,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(InstructionType::parse, tag(" "), Range2D::parse)
            .map(|(instruction_type, range)| Self {
                instruction_type,
                range,
            })
            .parse(input)
    }
}

struct Instructions(Vec<Instruction>);

impl Instructions {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list0(line_ending, Instruction::parse)
            .map(Self)
            .parse(input)
    }
}

struct GridBool(Box<[[bool; 1000]; 1000]>);

impl GridBool {
    fn new() -> Self {
        Self(Box::new([[false; 1000]; 1000]))
    }

    fn apply(&mut self, instruction: Instruction) {
        for x in instruction.range.min_x..=instruction.range.max_x {
            for y in instruction.range.min_y..=instruction.range.max_y {
                match instruction.instruction_type {
                    InstructionType::On => self.0[y][x] = true,
                    InstructionType::Off => self.0[y][x] = false,
                    InstructionType::Toggle => self.0[y][x] = !self.0[y][x],
                }
            }
        }
    }

    fn apply_all(&mut self, instructions: &Instructions) {
        for instruction in &instructions.0 {
            self.apply(*instruction)
        }
    }

    fn count_on(&self) -> usize {
        self.0.iter().flatten().filter(|b| **b).count()
    }
}

struct GridInt(Box<[[u32; 1000]; 1000]>);

impl GridInt {
    fn new() -> Self {
        Self(Box::new([[0; 1000]; 1000]))
    }

    fn apply(&mut self, instruction: Instruction) {
        for x in instruction.range.min_x..=instruction.range.max_x {
            for y in instruction.range.min_y..=instruction.range.max_y {
                match instruction.instruction_type {
                    InstructionType::On => self.0[y][x] += 1,
                    InstructionType::Off => self.0[y][x] = self.0[y][x].saturating_sub(1),
                    InstructionType::Toggle => self.0[y][x] += 2,
                }
            }
        }
    }

    fn apply_all(&mut self, instructions: &Instructions) {
        for instruction in &instructions.0 {
            self.apply(*instruction)
        }
    }

    fn total(&self) -> u32 {
        self.0.iter().flatten().sum()
    }
}

fn solve(path: &str) -> (usize, u32) {
    let input = read_to_string(path).unwrap();
    let mut grid = GridBool::new();
    let instructions = Instructions::parse(&input).unwrap().1;
    grid.apply_all(&instructions);
    let output_1 = grid.count_on();
    let mut grid = GridInt::new();
    grid.apply_all(&instructions);
    let output_2 = grid.total();
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
