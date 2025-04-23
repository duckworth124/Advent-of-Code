use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};
use winnow::{
    Parser, Result,
    ascii::{alpha1, dec_uint, newline},
    combinator::{opt, separated, seq},
};

#[derive(Debug)]
struct Tower(HashMap<String, Program>);

impl Tower {
    fn parse(mut input: &str) -> Result<Self> {
        separated(0.., Program::parse.map(|p| (p.name.clone(), p)), newline)
            .map(Self)
            .parse_next(&mut input)
    }

    fn bottom(&self) -> &str {
        self.0
            .keys()
            .find(|s| self.0.values().all(|p| !p.carrying.contains(s)))
            .unwrap()
    }

    fn total_weight(&self, program_name: &str) -> u32 {
        let program = &self.0[program_name];
        program.weight
            + program
                .carrying
                .iter()
                .map(|p| self.total_weight(p))
                .sum::<u32>()
    }

    fn required_weight_change(
        &self,
        program_name: &str,
        target_weight: Option<u32>,
    ) -> (String, u32) {
        let weights = self.0[program_name]
            .carrying
            .iter()
            .map(|s| self.total_weight(s))
            .collect_vec();

        if let Some(target) = target_weight {
            if weights.iter().all_equal() {
                return (
                    program_name.to_string(),
                    target - weights.iter().sum::<u32>(),
                );
            }
        };

        let mode_weight = {
            if weights[0] == weights[1] {
                weights[0]
            } else {
                weights[2]
            }
        };

        let target_program = self.0[program_name]
            .carrying
            .iter()
            .map(|s| (s, self.total_weight(s)))
            .find(|(_, w)| *w != mode_weight)
            .unwrap()
            .0;

        self.required_weight_change(&target_program, Some(mode_weight))
    }
}

#[derive(Debug)]
struct Program {
    name: String,
    carrying: Vec<String>,
    weight: u32,
}

impl Program {
    fn parse(input: &mut &str) -> Result<Self> {
        seq! {
            Self {
                name : alpha1.map(|s: &str| s.to_string()),
                _ :" (",
                weight: dec_uint,
                _ : ")",
                _: opt(" -> "),
                carrying: separated(0.., alpha1.map(|s: &str| s.to_string()), ", ")
            }

        }
        .parse_next(input)
    }
}

fn solve(tower: Tower) -> (String, u32) {
    let output_1 = tower.bottom().to_string();
    let (name, output_2) = tower.required_weight_change(&output_1, None);
    println!("{name}");
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let tower = Tower::parse(&input).unwrap();
    let (output_1, output_2) = solve(tower);
    println!("part 1: {output_1} part 2: {output_2}");
}
