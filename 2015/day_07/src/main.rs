use nom::{
    bytes::complete::tag,
    character::complete::{not_line_ending, u16},
    sequence::separated_pair,
    IResult, Parser,
};

struct GateId(usize);

struct Network<'a> {
    wires: Vec<&'a str>,
    inputs: Vec<Gate>,
}

impl<'a> Network<'a> {
    fn parse(input: &str) -> IResult<&str, Self> {
        todo!()
    }

    fn parse_literal(input: &str) -> IResult<&str, (Gate, &str)> {
        separated_pair(u16, tag(" -> "), not_line_ending)
            .map(|(l, s)| (Gate::Literal(l), s))
            .parse(input)
    }
}

enum Gate {
    Literal(u16),
    And(GateId, GateId),
    Or(GateId, GateId),
    Not(GateId),
    LShift(GateId, usize),
    RShift(GateId, usize),
}

fn main() {
    println!("Hello, world!");
}
