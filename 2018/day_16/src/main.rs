use itertools::Itertools;
use std::fs::read_to_string;
use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{delimited, separated},
    error::ContextError,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Operation {
    const fn all() -> [Self; 16] {
        [
            Self::Addr,
            Self::Addi,
            Self::Mulr,
            Self::Muli,
            Self::Banr,
            Self::Bani,
            Self::Borr,
            Self::Bori,
            Self::Setr,
            Self::Seti,
            Self::Gtir,
            Self::Gtri,
            Self::Gtrr,
            Self::Eqir,
            Self::Eqri,
            Self::Eqrr,
        ]
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    operation: Operation,
    out: usize,
    a: u64,
    b: u64,
}

impl Instruction {
    const fn apply(self, state: State) -> State {
        let (in1, in2) = match self.operation {
            Operation::Addr
            | Operation::Mulr
            | Operation::Banr
            | Operation::Borr
            | Operation::Setr
            | Operation::Gtrr
            | Operation::Eqrr => (state.get_register(self.a), state.get_register(self.b)),
            Operation::Addi
            | Operation::Muli
            | Operation::Bani
            | Operation::Bori
            | Operation::Gtri
            | Operation::Eqri => (state.get_register(self.a), self.b),

            Operation::Seti | Operation::Gtir | Operation::Eqir => {
                (self.a, state.get_register(self.b))
            }
        };

        let output_value = match self.operation {
            Operation::Addr | Operation::Addi => in1 + in2,
            Operation::Mulr | Operation::Muli => in1 * in2,
            Operation::Banr | Operation::Bani => in1 & in2,
            Operation::Borr | Operation::Bori => in1 | in2,
            Operation::Setr | Operation::Seti => in1,
            Operation::Gtir | Operation::Gtri | Operation::Gtrr => (in1 > in2) as u64,
            Operation::Eqir | Operation::Eqri | Operation::Eqrr => (in1 == in2) as u64,
        };
        let mut output = state;
        output.0[self.out] = output_value;
        output
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State([u64; 4]);

impl State {
    const fn get_register(self, register: u64) -> u64 {
        self.0[register as usize]
    }

    fn parse(input: &mut &str) -> Result<Self> {
        delimited(
            '[',
            separated(4, dec_uint::<&str, u64, ContextError>, ", "),
            ']',
        )
        .map(|v: Vec<u64>| Self(v.try_into().unwrap()))
        .parse_next(input)
    }
}

#[derive(Clone, Copy)]
struct Sample {
    op_code: usize,
    out: usize,
    a: u64,
    b: u64,
    before: State,
    after: State,
}

impl Sample {
    fn parse(input: &mut &str) -> Result<Self> {
        "Before: ".parse_next(input)?;
        let before = State::parse(input)?;
        '\n'.parse_next(input)?;
        let op_code: usize = dec_uint(input)?;
        ' '.parse_next(input)?;
        let a = dec_uint(input)?;
        ' '.parse_next(input)?;
        let b = dec_uint(input)?;
        ' '.parse_next(input)?;
        let out = dec_uint(input)?;
        '\n'.parse_next(input)?;
        "After:  ".parse_next(input)?;
        let after = State::parse(input)?;

        Ok(Self {
            op_code,
            out,
            a,
            b,
            before,
            after,
        })
    }

    fn possible_operations(self) -> Vec<Operation> {
        Operation::all()
            .into_iter()
            .filter(|op| {
                let instruction = Instruction {
                    operation: *op,
                    out: self.out,
                    a: self.a,
                    b: self.b,
                };
                instruction.apply(self.before) == self.after
            })
            .collect()
    }
}

fn solve(input: &str) -> (usize, u64) {
    let samples: Vec<Sample> = input
        .split("\n\n")
        .take_while(|l| !l.is_empty())
        .map(|s| Sample::parse.parse(s).unwrap())
        .collect();
    let output_1 = samples
        .iter()
        .map(|s| s.possible_operations().len())
        .filter(|n| *n >= 3)
        .count();

    let mut possible = [[true; 16]; 16];
    for sample in &samples {
        for i in 0..16 {
            if !sample.possible_operations().contains(&Operation::all()[i]) {
                possible[sample.op_code][i] = false;
            }
        }
    }
    let mut permutation: [Option<usize>; 16] = [None; 16];
    let mut i = 0;
    loop {
        if i == 16 {
            break;
        }
        let current = permutation[i];
        let new = current.map(|x| x + 1).unwrap_or_default();
        if new == 16 {
            permutation[i] = None;
            i = i.checked_sub(1).unwrap();
            continue;
        }
        permutation[i] = Some(new);
        if !possible[i][new] {
            continue;
        }
        if !permutation.iter().copied().flatten().all_unique() {
            continue;
        }
        i += 1
    }
    let permutation: [Operation; 16] = permutation.map(|i| Operation::all()[i.unwrap()]);
    let instructions: Vec<Instruction> = input
        .split_once("\n\n\n\n")
        .unwrap()
        .1
        .lines()
        .map(|s| {
            s.split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|v| Instruction {
            operation: permutation[v[0] as usize],
            a: v[1],
            b: v[2],
            out: v[3] as usize,
        })
        .collect();

    let mut state = State([0; 4]);
    for instruction in instructions {
        state = instruction.apply(state);
    }
    let output_2 = state.0[0];

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
