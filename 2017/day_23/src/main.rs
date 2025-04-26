use std::collections::HashMap;
use std::fs::read_to_string;
use winnow::ascii::newline;
use winnow::ascii::{alpha1, dec_int};
use winnow::combinator::alt;
use winnow::combinator::separated;
use winnow::{Parser, Result};

#[derive(Debug, Clone)]
enum Instruction {
    Set(String, Value),
    Sub(String, Value),
    Mul(String, Value),
    Jnz(Value, Value),
}

impl Instruction {
    fn parse_set(input: &mut &str) -> Result<Self> {
        "set ".parse_next(input)?;
        let reg = alpha1(input)?;
        ' '.parse_next(input)?;
        let value = Value::parse(input)?;
        Ok(Self::Set(reg.to_string(), value))
    }

    fn parse_mul(input: &mut &str) -> Result<Self> {
        "mul ".parse_next(input)?;
        let reg = alpha1(input)?;
        ' '.parse_next(input)?;
        let value = Value::parse(input)?;
        Ok(Self::Mul(reg.to_string(), value))
    }

    fn parse_sub(input: &mut &str) -> Result<Self> {
        "sub ".parse_next(input)?;
        let reg = alpha1(input)?;
        ' '.parse_next(input)?;
        let value = Value::parse(input)?;
        Ok(Self::Sub(reg.to_string(), value))
    }
    fn parse_jnz(input: &mut &str) -> Result<Self> {
        "jnz ".parse_next(input)?;
        let value_1 = Value::parse(input)?;
        ' '.parse_next(input)?;
        let value_2 = Value::parse(input)?;
        Ok(Self::Jnz(value_1, value_2))
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((
            Self::parse_set,
            Self::parse_mul,
            Self::parse_jnz,
            Self::parse_sub,
        ))
        .parse_next(input)
    }
}

#[derive(Debug, Clone)]
enum Value {
    Literal(i64),
    Register(String),
}

impl Value {
    fn parse_literal(input: &mut &str) -> Result<Self> {
        dec_int(input).map(Self::Literal)
    }

    fn parse_register(input: &mut &str) -> Result<Self> {
        alpha1(input).map(|s| Self::Register(s.to_string()))
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((Self::parse_literal, Self::parse_register)).parse_next(input)
    }
}

#[derive(Clone, Debug)]
struct State {
    registers: HashMap<String, i64>,
    instructions: Vec<Instruction>,
    pc: usize,
}

impl State {
    fn step(&mut self) -> bool {
        let instruction = &self.instructions[self.pc];
        match instruction {
            Instruction::Set(reg, value) => {
                self.registers.insert(reg.to_string(), self.eval(value));
            }
            Instruction::Mul(reg, value) => {
                *self.registers.entry(reg.to_string()).or_default() *= self.eval(value)
            }
            Instruction::Sub(reg, value) => {
                *self.registers.entry(reg.to_string()).or_default() -= self.eval(value)
            }
            Instruction::Jnz(value_1, value_2) => {
                if self.eval(value_1) != 0 {
                    let new_pc = (self.pc as i64) + self.eval(value_2);
                    if new_pc < 0 || new_pc >= self.instructions.len() as i64 {
                        return false;
                    }

                    self.pc = new_pc as usize
                } else {
                    self.pc += 1
                }
            }
        };

        if !matches!(instruction, Instruction::Jnz(_, _)) {
            self.pc += 1;
            if self.pc >= self.instructions.len() {
                return false;
            }
        }

        true
    }

    fn eval(&self, value: &Value) -> i64 {
        match value {
            Value::Literal(v) => *v,
            Value::Register(r) => self.registers.get(r).copied().unwrap_or_default(),
        }
    }
}

fn solve(mut input: &str) -> (u32, usize) {
    let instructions: Vec<Instruction> = separated(0.., Instruction::parse, newline)
        .parse_next(&mut input)
        .unwrap();
    let mut state = State {
        registers: HashMap::new(),
        instructions,
        pc: 0,
    };

    let mut output_1 = 0;
    loop {
        let instruction = &state.instructions[state.pc];
        if matches!(instruction, Instruction::Mul(_, _)) {
            output_1 += 1
        }
        if !state.step() {
            break;
        }
    }

    //the program counts the numbers b for 107900 <= b <= 124900 in steps of 17 that are composite
    let output_2: usize = (107900..=124900)
        .step_by(17)
        .map(|n| (2..n).filter(|p| n % *p == 0).count())
        .filter(|n| *n > 0)
        .count();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (ouput_1, output_2) = solve(&input);
    println!("part 1: {ouput_1} part 2: {output_2}")
}
