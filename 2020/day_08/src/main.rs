use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::separated_list0,
    sequence::tuple,
    IResult, Parser,
};
use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    fn parse_nop(input: &str) -> IResult<&str, Self> {
        tuple((tag("nop "), i32))
            .map(|x| x.1)
            .map(Self::Nop)
            .parse(input)
    }

    fn parse_acc(input: &str) -> IResult<&str, Self> {
        tuple((tag("acc "), i32))
            .map(|(_, n)| n)
            .map(Self::Acc)
            .parse(input)
    }

    fn parse_jmp(input: &str) -> IResult<&str, Self> {
        tuple((tag("jmp "), i32))
            .map(|x| x.1)
            .map(Self::Jmp)
            .parse(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_nop, Self::parse_jmp, Self::parse_acc)).parse(input)
    }

    fn parse_multi(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(newline, Self::parse).parse(input)
    }

    const fn flip(&mut self) {
        *self = match self {
            Self::Nop(n) => Self::Jmp(*n),
            Self::Jmp(n) => Self::Nop(*n),
            _ => *self,
        }
    }
}

fn run(instructions: &[Instruction]) -> (i32, bool) {
    let mut pc = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(pc) {
            return (acc, false);
        }
        if pc >= instructions.len() {
            return (acc, true);
        }
        let current = instructions[pc];
        match current {
            Instruction::Nop(_) => {}
            Instruction::Acc(n) => acc += n,
            Instruction::Jmp(n) => pc = (pc as i32 + n) as usize,
        }

        if !matches!(current, Instruction::Jmp(_)) {
            pc += 1
        }
    }
}

fn find_halt(instructions: &mut [Instruction]) -> i32 {
    for i in 0..instructions.len() {
        if matches!(instructions[i], Instruction::Acc(_)) {
            continue;
        }
        instructions[i].flip();
        let (acc, halted) = run(instructions);
        if halted {
            return acc;
        };

        instructions[i].flip()
    }

    panic!()
}

fn solve(path: &str) -> (i32, i32) {
    let input = read_to_string(path).unwrap();
    let mut instructions = Instruction::parse_multi(&input).unwrap().1;
    let output_1 = run(&instructions).0;
    let output_2 = find_halt(&mut instructions);
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
