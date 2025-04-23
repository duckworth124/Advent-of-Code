use std::collections::HashMap;
use std::fs::read_to_string;
use winnow::ascii::{alpha1, dec_int, newline};
use winnow::combinator::{alt, repeat, separated};
use winnow::{Parser, Result};

#[derive(Default, Debug)]
struct State {
    registers: HashMap<String, i32>,
}

impl State {
    fn apply_instruction(&mut self, instruction: Instruction) {
        let (compare_reg, comparison, compare_val) = instruction.condition;
        let reg_val = self
            .registers
            .get(&compare_reg)
            .copied()
            .unwrap_or_default();

        let condition = match comparison {
            Comparison::LE => reg_val <= compare_val,
            Comparison::LT => reg_val < compare_val,
            Comparison::GE => reg_val >= compare_val,
            Comparison::GT => reg_val > compare_val,
            Comparison::EQ => reg_val == compare_val,
            Comparison::NE => reg_val != compare_val,
        };

        if condition {
            *self.registers.entry(instruction.register).or_default() += instruction.change
        }
    }
}

struct Instruction {
    register: String,
    change: i32,
    condition: (String, Comparison, i32),
}

impl Instruction {
    fn parse(input: &mut &str) -> Result<Self> {
        let register = alpha1.parse_next(input)?;
        ' '.parse_next(input)?;
        let inc = alt(("inc".value(true), "dec".value(false))).parse_next(input)?;
        ' '.parse_next(input)?;
        let change: i32 = dec_int(input)?;
        let change = if inc { change } else { -change };
        " if ".parse_next(input)?;
        let compare_reg = alpha1(input)?;
        ' '.parse_next(input)?;
        let comparison = Comparison::parse(input)?;
        ' '.parse_next(input)?;
        let compare_val = dec_int(input)?;

        Ok(Self {
            register: register.to_string(),
            change,
            condition: (compare_reg.to_string(), comparison, compare_val),
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Comparison {
    LE,
    LT,
    GE,
    GT,
    EQ,
    NE,
}

impl Comparison {
    fn parse(input: &mut &str) -> Result<Self> {
        alt((
            ">=".value(Self::GE),
            ">".value(Self::GT),
            "<=".value(Self::LE),
            "<".value(Self::LT),
            "==".value(Self::EQ),
            "!=".value(Self::NE),
        ))
        .parse_next(input)
    }
}

fn solve(mut input: &str) -> (i32, i32) {
    let mut state = State::default();
    let instructions: Vec<Instruction> = separated(0.., Instruction::parse, newline)
        .parse_next(&mut input)
        .unwrap();
    let mut output_2 = 0;
    for instruction in instructions {
        state.apply_instruction(instruction);
        output_2 = output_2.max(*state.registers.values().max().unwrap());
    }
    let output_1 = state.registers.values().copied().max().unwrap().max(0);

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
