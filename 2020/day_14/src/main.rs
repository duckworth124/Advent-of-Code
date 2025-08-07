use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs::read_to_string,
};

use winnow::{
    ascii::dec_uint,
    combinator::{alt, delimited, repeat},
    Parser, Result,
};

#[derive(Clone)]
struct Mask([Option<bool>; 36]);
impl Mask {
    fn apply(&self, mut value: u64) -> u64 {
        for i in 0..36 {
            let bit = self.0[35 - i];
            let bit = if let Some(b) = bit {
                b
            } else {
                continue;
            };
            if bit {
                value |= 1 << i
            } else {
                value &= !(1 << i)
            }
        }

        value
    }

    fn parse(input: &mut &str) -> Result<Self> {
        repeat(
            36,
            alt((
                '1'.value(Some(true)),
                '0'.value(Some(false)),
                'X'.value(None),
            )),
        )
        .map(|v: Vec<Option<bool>>| v.try_into().unwrap())
        .map(Self)
        .parse_next(input)
    }
}

#[derive(Clone)]
enum Instruction {
    SetMask(Mask),
    Write(u64, u64),
}

impl Instruction {
    fn parse_set_mask(input: &mut &str) -> Result<Self> {
        "mask = ".parse_next(input)?;
        Mask::parse.map(Self::SetMask).parse_next(input)
    }

    fn parse_write(input: &mut &str) -> Result<Self> {
        "mem".parse_next(input)?;
        let addr: u64 = delimited('[', dec_uint, ']').parse_next(input)?;
        " = ".parse_next(input)?;
        let value: u64 = dec_uint(input)?;
        Ok(Self::Write(addr, value))
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((Self::parse_set_mask, Self::parse_write)).parse_next(input)
    }
}

struct State {
    mask: Mask,
    mem: HashMap<u64, u64>,
}

impl State {
    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(m) => self.mask = m,
            Instruction::Write(addr, value) => {
                self.mem.insert(addr, self.mask.apply(value));
            }
        }
    }

    fn mem_sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Region([Option<bool>; 36]);

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            let c = match b {
                Some(true) => '1',
                Some(false) => '0',
                None => 'X',
            };
            write!(f, "{c}")?
        }
        Ok(())
    }
}

impl Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Region {
    fn size(&self) -> u64 {
        1 << self.0.iter().filter(|o| o.is_none()).count()
    }

    fn is_subset(&self, rhs: &Self) -> bool {
        self.0.iter().zip(&rhs.0).all(|x| match x {
            (_, None) => true,
            (None, _) => false,
            (x, y) => x == y,
        })
    }

    fn is_disjoint(&self, rhs: &Self) -> bool {
        self.0
            .iter()
            .zip(&rhs.0)
            .any(|(x, y)| matches!((x, y), (Some(x), Some(y)) if x != y))
    }

    fn split(mut self, rhs: &Self) -> (Self, Self) {
        let i = self
            .0
            .iter()
            .zip(&rhs.0)
            .position(|(x, y)| x.is_none() && y.is_some())
            .unwrap();

        self.0[i] = Some(false);
        let l = self.clone();
        self.0[i] = Some(true);
        (l, self)
    }

    fn new(addr: u64, mask: &Mask) -> Self {
        let v: Vec<Option<bool>> = mask
            .0
            .iter()
            .copied()
            .rev()
            .enumerate()
            .rev()
            .map(|(i, b)| {
                let b = b?;
                if b {
                    Some(true)
                } else {
                    Some(addr & (1 << i) > 0)
                }
            })
            .collect();

        Self(v.try_into().unwrap())
    }
}

struct CompressedState {
    mask: Mask,
    mem: Vec<(Region, u64)>,
}

impl CompressedState {
    fn new() -> Self {
        let mask = Mask([None; 36]);
        let mem = vec![(Region([None; 36]), 0)];
        Self { mask, mem }
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(m) => {
                self.mask = m;
            }
            Instruction::Write(addr, new_value) => {
                let new_region = Region::new(addr, &self.mask);
                let mut new_mem = vec![];
                while let Some((current_region, current_value)) = self.mem.pop() {
                    if current_region.is_subset(&new_region) {
                        new_mem.push((current_region, new_value));
                        continue;
                    }
                    if current_region.is_disjoint(&new_region) {
                        new_mem.push((current_region, current_value));
                        continue;
                    }

                    let (l, r) = current_region.split(&new_region);
                    self.mem.push((l, current_value));
                    self.mem.push((r, current_value));
                }

                self.mem = new_mem
            }
        }
    }

    fn mem_sum(&self) -> u64 {
        self.mem.iter().map(|(r, v)| r.size() * v).sum()
    }

    #[allow(unused)]
    fn dbg_mem(&self) {
        println!("current mem:");
        for (region, value) in &self.mem {
            if *value != 0 {
                println!("{region} = {value}")
            }
        }
    }
}

fn solve(input: &str) -> (u64, u64) {
    let mut state = State {
        mask: Mask([None; 36]),
        mem: HashMap::new(),
    };
    let mut compressed_state = CompressedState::new();

    for line in input.lines() {
        let instruction = Instruction::parse.parse(line).unwrap();
        state.apply_instruction(instruction.clone());
        compressed_state.apply_instruction(instruction);
    }

    (state.mem_sum(), compressed_state.mem_sum())
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
}
