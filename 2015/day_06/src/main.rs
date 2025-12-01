use std::fs::read_to_string;

use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{alt, separated, separated_pair},
    Parser, Result,
};

#[derive(Clone, Copy)]
struct Range2D {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

impl Range2D {
    fn parse(input: &mut &str) -> Result<Self> {
        separated_pair(
            separated_pair(dec_uint, ",", dec_uint),
            " through ",
            separated_pair(dec_uint, ",", dec_uint),
        )
        .map(|((min_x, min_y), (max_x, max_y))| Self {
            min_x,
            min_y,
            max_x,
            max_y,
        })
        .parse_next(input)
    }
}

#[derive(Clone, Copy)]
enum InstructionType {
    On,
    Off,
    Toggle,
}

impl InstructionType {
    fn parse_on(input: &mut &str) -> Result<Self> {
        ("turn on").value(Self::On).parse_next(input)
    }

    fn parse_off(input: &mut &str) -> Result<Self> {
        ("turn off").value(Self::Off).parse_next(input)
    }

    fn parse_toggle(input: &mut &str) -> Result<Self> {
        ("toggle").value(Self::Toggle).parse_next(input)
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((Self::parse_on, Self::parse_off, Self::parse_toggle)).parse_next(input)
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    instruction_type: InstructionType,
    range: Range2D,
}

impl Instruction {
    fn parse(input: &mut &str) -> Result<Self> {
        separated_pair(InstructionType::parse, " ", Range2D::parse)
            .map(|(instruction_type, range)| Self {
                instruction_type,
                range,
            })
            .parse_next(input)
    }
}

struct Instructions(Vec<Instruction>);

impl Instructions {
    fn parse(input: &mut &str) -> Result<Self> {
        separated(0.., Instruction::parse, line_ending)
            .map(Self)
            .parse_next(input)
    }
}

struct GridBool([[bool; 1000]; 1000]);

impl GridBool {
    #[allow(clippy::large_stack_frames)]
    const fn new() -> Self {
        Self([[false; 1000]; 1000])
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

struct GridInt([[u32; 1000]; 1000]);

impl GridInt {
    #[allow(clippy::large_stack_frames)]
    const fn new() -> Self {
        Self([[0; 1000]; 1000])
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

#[allow(clippy::large_stack_frames)]
fn solve(path: &str) -> (usize, u32) {
    let input = read_to_string(path).unwrap();
    let mut grid = GridBool::new();
    let instructions = Instructions::parse.parse(input.trim()).unwrap();
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
