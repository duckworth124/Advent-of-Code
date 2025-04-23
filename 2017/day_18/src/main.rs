use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};
use winnow::{
    Parser, Result,
    ascii::{alpha1, dec_int, newline},
    combinator::{alt, separated},
};

struct State {
    registers: HashMap<String, i64>,
    instructions: Vec<Instruction>,
    pc: usize,
    sound: Option<i64>,
    recovered: Option<i64>,
}

impl State {
    fn step(&mut self) -> bool {
        let instruction = &self.instructions[self.pc];
        match instruction {
            Instruction::Snd(value) => self.sound = Some(self.eval(value)),
            Instruction::Set(reg, value) => {
                self.registers.insert(reg.to_string(), self.eval(value));
            }
            Instruction::Add(reg, value) => {
                *self.registers.entry(reg.to_string()).or_default() += self.eval(value)
            }
            Instruction::Mul(reg, value) => {
                *self.registers.entry(reg.to_string()).or_default() *= self.eval(value)
            }
            Instruction::Mod(reg, value) => {
                *self.registers.entry(reg.to_string()).or_default() %= self.eval(value)
            }
            Instruction::Rcv(value) => {
                if self.registers.get(value).copied().unwrap_or_default() != 0 {
                    self.recovered = self.sound
                }
            }
            Instruction::Jgz(value_1, value_2) => {
                if self.eval(value_1) > 0 {
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

        if !matches!(instruction, Instruction::Jgz(_, _)) {
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

#[derive(Debug, Clone)]
enum Instruction {
    Snd(Value),
    Set(String, Value),
    Add(String, Value),
    Mul(String, Value),
    Mod(String, Value),
    Rcv(String),
    Jgz(Value, Value),
}

impl Instruction {
    fn parse_snd(input: &mut &str) -> Result<Self> {
        "snd ".parse_next(input)?;
        let value = Value::parse(input)?;
        Ok(Self::Snd(value))
    }

    fn parse_set(input: &mut &str) -> Result<Self> {
        "set ".parse_next(input)?;
        let reg = alpha1(input)?;
        ' '.parse_next(input)?;
        let value = Value::parse(input)?;
        Ok(Self::Set(reg.to_string(), value))
    }

    fn parse_add(input: &mut &str) -> Result<Self> {
        "add ".parse_next(input)?;
        let reg = alpha1(input)?;
        ' '.parse_next(input)?;
        let value = Value::parse(input)?;
        Ok(Self::Add(reg.to_string(), value))
    }

    fn parse_mul(input: &mut &str) -> Result<Self> {
        "mul ".parse_next(input)?;
        let reg = alpha1(input)?;
        ' '.parse_next(input)?;
        let value = Value::parse(input)?;
        Ok(Self::Mul(reg.to_string(), value))
    }

    fn parse_mod(input: &mut &str) -> Result<Self> {
        "mod ".parse_next(input)?;
        let reg = alpha1(input)?;
        ' '.parse_next(input)?;
        let value = Value::parse(input)?;
        Ok(Self::Mod(reg.to_string(), value))
    }

    fn parse_rcv(input: &mut &str) -> Result<Self> {
        "rcv ".parse_next(input)?;
        let reg = alpha1(input)?;
        Ok(Self::Rcv(reg.to_string()))
    }

    fn parse_jgz(input: &mut &str) -> Result<Self> {
        "jgz ".parse_next(input)?;
        let value_1 = Value::parse(input)?;
        ' '.parse_next(input)?;
        let value_2 = Value::parse(input)?;
        Ok(Self::Jgz(value_1, value_2))
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((
            Self::parse_snd,
            Self::parse_set,
            Self::parse_add,
            Self::parse_mul,
            Self::parse_mod,
            Self::parse_rcv,
            Self::parse_jgz,
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

struct Duet {
    instructions: Vec<Instruction>,
    program_0: Program,
    program_1: Program,
    p1_send_count: u32,
}

impl Duet {
    fn step(&mut self) -> bool {
        match self.program_0.status {
            ProgramStatus::Active => {
                self.step_program_0();

                return true;
            }

            ProgramStatus::Receiving if !self.program_0.channel.is_empty() => {
                self.step_program_0();
                return true;
            }

            _ => {}
        };

        match self.program_1.status {
            ProgramStatus::Active => {
                self.step_program_1();

                return true;
            }

            ProgramStatus::Receiving if !self.program_1.channel.is_empty() => {
                self.step_program_1();
                return true;
            }

            _ => {}
        };

        false
    }

    fn step_program_0(&mut self) {
        let sent = self.program_0.step(&self.instructions);
        if let Some(value) = sent {
            self.program_1.channel.push_back(value);
        }
    }

    fn step_program_1(&mut self) {
        let sent = self.program_1.step(&self.instructions);
        if let Some(value) = sent {
            self.program_0.channel.push_back(value);
            self.p1_send_count += 1
        }
    }
}

struct Program {
    status: ProgramStatus,
    pc: usize,
    registers: HashMap<String, i64>,
    channel: VecDeque<i64>,
}

impl Program {
    fn step(&mut self, instructions: &[Instruction]) -> Option<i64> {
        let instruction = &instructions[self.pc];
        let mut output = None;
        match instruction {
            Instruction::Snd(value) => output = Some(self.eval(value)),
            Instruction::Set(reg, value) => {
                self.registers.insert(reg.to_string(), self.eval(value));
            }
            Instruction::Add(reg, value) => {
                *self.registers.entry(reg.to_string()).or_default() += self.eval(value)
            }
            Instruction::Mul(reg, value) => {
                *self.registers.entry(reg.to_string()).or_default() *= self.eval(value)
            }
            Instruction::Mod(reg, value) => {
                *self.registers.entry(reg.to_string()).or_default() %= self.eval(value)
            }
            Instruction::Rcv(reg) => match self.channel.pop_front() {
                Some(value) => {
                    self.registers.insert(reg.to_string(), value);
                }
                None => {
                    self.status = ProgramStatus::Receiving;
                    return None;
                }
            },
            Instruction::Jgz(value_1, value_2) => {
                if self.eval(value_1) > 0 {
                    let new_pc = (self.pc as i64) + self.eval(value_2);
                    if new_pc < 0 || new_pc >= instructions.len() as i64 {
                        self.status = ProgramStatus::Terminated;
                        return None;
                    }

                    self.pc = new_pc as usize
                } else {
                    self.pc += 1
                }
            }
        };

        if !matches!(instruction, Instruction::Jgz(_, _)) {
            self.pc += 1;
            if self.pc >= instructions.len() {
                self.status = ProgramStatus::Terminated
            }
        }

        output
    }

    fn eval(&self, value: &Value) -> i64 {
        match value {
            Value::Literal(v) => *v,
            Value::Register(r) => self.registers.get(r).copied().unwrap_or_default(),
        }
    }
}

enum ProgramStatus {
    Active,
    Receiving,
    Terminated,
}

fn solve(mut input: &str) -> (i64, u32) {
    let instructions: Vec<Instruction> = separated(0.., Instruction::parse, newline)
        .parse_next(&mut input)
        .unwrap();

    let mut state = State {
        instructions: instructions.clone(),
        registers: HashMap::new(),
        pc: 0,
        sound: None,
        recovered: None,
    };

    let output_1 = loop {
        state.step();
        if let Some(output) = state.recovered {
            break output;
        }
    };

    let mut duet = Duet {
        instructions,
        program_0: Program {
            status: ProgramStatus::Active,
            pc: 0,
            registers: HashMap::new(),
            channel: VecDeque::new(),
        },
        program_1: Program {
            status: ProgramStatus::Active,
            pc: 0,
            registers: HashMap::new(),
            channel: VecDeque::new(),
        },
        p1_send_count: 0,
    };

    duet.program_1.registers.insert("p".to_string(), 1);
    while duet.step() {}
    let output_2 = duet.p1_send_count;
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
