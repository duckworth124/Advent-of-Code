use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string, mem};
use winnow::{
    ascii::{alphanumeric0, dec_uint, line_ending, till_line_ending},
    combinator::{alt, separated},
    PResult, Parser,
};

#[derive(Clone, Copy)]
enum Connection {
    Literal(bool),
    Gate(Gate, usize, usize),
}

#[derive(Clone, Copy)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    fn parse(input: &mut &str) -> PResult<Self> {
        alt((Self::parse_and, Self::parse_or, Self::parse_xor)).parse_next(input)
    }

    fn parse_and(input: &mut &str) -> PResult<Self> {
        ("AND").value(Self::And).parse_next(input)
    }
    fn parse_or(input: &mut &str) -> PResult<Self> {
        ("OR").value(Self::Or).parse_next(input)
    }
    fn parse_xor(input: &mut &str) -> PResult<Self> {
        ("XOR").value(Self::Xor).parse_next(input)
    }

    fn eval(self, x: bool, y: bool) -> bool {
        match self {
            Self::Or => x | y,
            Self::And => x & y,
            Self::Xor => x ^ y,
        }
    }
}

struct Network {
    wires: Vec<String>,
    connections: Vec<Connection>,
    indices: HashMap<String, usize>,
}

impl Network {
    fn parse(input: &str) -> Self {
        let mut wires = vec![];
        let mut indices = HashMap::new();

        let (literals, gates): (Vec<(&str, Connection)>, Vec<_>) = (
            separated(0.., Self::parse_literal, line_ending),
            "\n\n",
            separated(0.., Self::parse_gate, line_ending),
        )
            .map(|(l, _, g)| (l, g))
            .parse(input)
            .unwrap();

        let literals: Vec<(usize, Connection)> = literals
            .into_iter()
            .map(|(s, c)| (index_of(s, &mut indices, &mut wires), c))
            .collect();

        let gates: Vec<(usize, Connection)> = gates
            .into_iter()
            .map(|(x, y, z, g)| {
                (
                    index_of(z, &mut indices, &mut wires),
                    Connection::Gate(
                        g,
                        index_of(x, &mut indices, &mut wires),
                        index_of(y, &mut indices, &mut wires),
                    ),
                )
            })
            .collect();

        let connections = literals
            .into_iter()
            .chain(gates)
            .sorted_unstable_by_key(|t| t.0)
            .map(|t| t.1)
            .collect_vec();
        let wires = wires.into_iter().map(|s| s.to_string()).collect_vec();
        let indices = indices
            .into_iter()
            .map(|(s, n)| (s.to_string(), n))
            .collect();

        Self {
            wires,
            connections,
            indices,
        }
    }

    fn parse_gate<'a>(input: &mut &'a str) -> PResult<(&'a str, &'a str, &'a str, Gate)> {
        (
            alphanumeric0,
            (' '),
            Gate::parse,
            (' '),
            alphanumeric0,
            (" -> "),
            till_line_ending,
        )
            .map(|(x, _, g, _, y, _, z)| (x, y, z, g))
            .parse_next(input)
    }

    fn parse_literal<'a>(input: &mut &'a str) -> PResult<(&'a str, Connection)> {
        (alphanumeric0, (": "), dec_uint)
            .map(|(s, _, n): (&str, _, u8)| (s, Connection::Literal(n != 0)))
            .parse_next(input)
    }

    fn run(&self) -> Option<Vec<bool>> {
        let mut outputs = vec![None; self.wires.len()];
        for i in 0..outputs.len() {
            self.calculate(&mut outputs, i, vec![])?;
        }

        Some(outputs.into_iter().flatten().collect_vec())
    }

    fn calculate(
        &self,
        outputs: &mut [Option<bool>],
        i: usize,
        mut calls: Vec<usize>,
    ) -> Option<bool> {
        if calls.contains(&i) {
            return None;
        }

        calls.push(i);
        if let Some(output) = outputs[i] {
            return Some(output);
        }
        let connection = self.connections[i];
        let output = match connection {
            Connection::Gate(g, x, y) => g.eval(
                self.calculate(outputs, x, calls.clone())?,
                self.calculate(outputs, y, calls.clone())?,
            ),
            Connection::Literal(b) => b,
        };

        outputs[i] = Some(output);
        Some(output)
    }

    fn swap(&mut self, x: &str, y: &str) {
        let x = self.indices[x];
        let y = self.indices[y];
        let l = x.min(y);
        let u = x.max(y);
        let (left, right) = self.connections.split_at_mut(u);
        mem::swap(&mut left[l], right.first_mut().unwrap())
    }

    fn is_addition(&self) -> bool {
        let outputs = match self.run() {
            Some(v) => v,
            None => return false,
        };
        let x = self.get_output(&outputs, 'x');
        let y = self.get_output(&outputs, 'y');
        let z = self.get_output(&outputs, 'z');
        x + y == z
    }

    fn get_output(&self, outputs: &[bool], wire: char) -> u64 {
        self.wires
            .iter()
            .enumerate()
            .filter(|(_, s)| s.starts_with(wire))
            .sorted_unstable_by_key(|(_, s)| *s)
            .rev()
            .map(|(i, _)| outputs[i])
            .fold(0, |acc, b| acc * 2 + b as u64)
    }

    fn find_swaps(&mut self) {
        for x in 0..self.wires.len() {
            for y in x + 1..self.wires.len() {
                let x = self.wires[x].clone();
                let y = self.wires[y].clone();
                if x.starts_with('x') | x.starts_with('y') | y.starts_with('x') | y.starts_with('y')
                {
                    continue;
                }
                self.swap(&x, &y);
                if self.is_addition() {
                    println!("{x} {y}");
                }

                self.swap(&x, &y)
            }
        }
    }
}

fn index_of<'a>(
    s: &'a str,
    indices: &mut HashMap<&'a str, usize>,
    wires: &mut Vec<&'a str>,
) -> usize {
    if let Some(output) = indices.get(s) {
        return *output;
    }
    wires.push(s);
    indices.insert(s, wires.len() - 1);
    wires.len() - 1
}

pub fn solve(path: &str) -> (u64, String) {
    println!("{path}");
    let input = read_to_string(path).unwrap();
    let mut network = Network::parse(input.trim());
    let outputs = network.run().unwrap();
    let ouptut_1 = network.get_output(&outputs, 'z');

    network.swap("z39", "tnc");
    network.swap("fhg", "z17");
    network.swap("z10", "vcf");

    network.find_swaps();

    let output_2 = ["z39", "tnc", "fhg", "z17", "z10", "vcf", "dvb", "fsq"]
        .into_iter()
        .sorted()
        .join(",");

    let outputs = network.run().unwrap();
    let x = network.get_output(&outputs, 'x');
    let y = network.get_output(&outputs, 'y');
    println!("{}", x + y);

    (ouptut_1, output_2)
}
