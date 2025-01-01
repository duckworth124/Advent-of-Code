use std::{collections::HashMap, fs::read_to_string, u64, usize};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, u64},
    multi::separated_list0,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Clone, Copy)]
struct Mask {
    or: u64,
    and: u64,
}

impl Mask {
    fn new(input: &str) -> Self {
        let mut or = 0;
        let mut and = (1 << 36) - 1;
        for (i, c) in input.chars().rev().enumerate() {
            match c {
                'X' => {}
                '0' => and &= !(1 << i),

                '1' => or |= 1 << i,
                _ => panic!("unrecognized character: {c}"),
            }
        }

        Self { or, and }
    }

    fn apply(self, input: u64) -> u64 {
        input & self.and | self.or
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    MaskSet(Mask),
    MemSet { index: usize, value: u64 },
}

impl Instruction {
    fn parse_mask_set(input: &str) -> IResult<&str, Self> {
        tuple((tag("mask = "), not_line_ending))
            .map(|x| x.1)
            .map(Mask::new)
            .map(Self::MaskSet)
            .parse(input)
    }

    fn parse_mem_set(input: &str) -> IResult<&str, Self> {
        tuple((tag("mem["), u64, tag("] = "), u64))
            .map(|(_, i, _, v)| (i as usize, v))
            .map(|(index, value)| Self::MemSet { index, value })
            .parse(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Self::parse_mask_set, Self::parse_mem_set)).parse(input)
    }
}

struct Instructions(Vec<Instruction>);

impl Instructions {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list0(newline, Instruction::parse)
            .map(Self)
            .parse(input)
    }

    fn apply(&self) -> HashMap<usize, u64> {
        self.0
            .iter()
            .fold(
                (
                    HashMap::new(),
                    Mask {
                        or: 0,
                        and: (1 << 36) - 1,
                    },
                ),
                |(mut map, mut mask), i| {
                    match i {
                        Instruction::MaskSet(m) => mask = *m,
                        Instruction::MemSet { index, value } => {
                            map.insert(*index, mask.apply(*value));
                        }
                    };

                    (map, mask)
                },
            )
            .0
    }
}

pub fn solve(path: &str) -> u64 {
    let input = read_to_string(path).unwrap();
    let instructions = Instructions::parse(&input).unwrap().1;
    instructions.apply().values().sum()
}
