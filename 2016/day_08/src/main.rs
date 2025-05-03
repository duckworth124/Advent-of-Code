use std::{fmt::Display, fs::read_to_string};
use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{alt, separated_pair},
};

struct Grid([[bool; 50]; 6]);

impl Grid {
    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rect(width, height) => {
                self.0[..height]
                    .iter_mut()
                    .for_each(|row| row[..width].fill(true));
            }
            Instruction::RotateRow { row, amount } => {
                self.0[row].rotate_right(amount);
            }
            Instruction::RotateCol { col, amount } => {
                let mut column: Vec<bool> = self.0.iter().map(|row| row[col]).collect();
                column.rotate_right(amount);
                column
                    .into_iter()
                    .zip(self.0.iter_mut())
                    .for_each(|(b, row)| row[col] = b);
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for b in row {
                if *b { write!(f, "#")? } else { write!(f, ".")? }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

enum Instruction {
    Rect(usize, usize),
    RotateRow { row: usize, amount: usize },
    RotateCol { col: usize, amount: usize },
}

impl Instruction {
    fn parse(input: &mut &str) -> Result<Self> {
        alt((
            Self::parse_rect,
            Self::parse_rotate_row,
            Self::parse_rotate_col,
        ))
        .parse_next(input)
    }

    fn parse_rect(input: &mut &str) -> Result<Self> {
        "rect ".parse_next(input)?;
        separated_pair(dec_uint, 'x', dec_uint)
            .map(|(width, height): (usize, usize)| Self::Rect(width, height))
            .parse_next(input)
    }

    fn parse_rotate_row(input: &mut &str) -> Result<Self> {
        "rotate row y=".parse_next(input)?;
        let row: usize = dec_uint(input)?;
        " by ".parse_next(input)?;
        let amount: usize = dec_uint(input)?;
        Ok(Self::RotateRow { row, amount })
    }

    fn parse_rotate_col(input: &mut &str) -> Result<Self> {
        "rotate column x=".parse_next(input)?;
        let col: usize = dec_uint(input)?;
        " by ".parse_next(input)?;
        let amount: usize = dec_uint(input)?;
        Ok(Self::RotateCol { col, amount })
    }
}

fn solve(input: &str) -> (usize, String) {
    let mut grid = Grid([[false; 50]; 6]);
    input
        .lines()
        .map(|mut l| Instruction::parse(&mut l).unwrap())
        .for_each(|i| grid.apply_instruction(i));

    let output_1 = grid.0.iter().flatten().filter(|b| **b).count();
    let output_2 = grid.to_string();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1}");
    println!("part 2:\n{output_2}")
}
