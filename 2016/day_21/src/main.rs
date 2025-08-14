use std::fs::read_to_string;

use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{alt, opt},
    token::any,
};

#[derive(Clone, Copy)]
enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOn(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instruction {
    fn apply(self, chars: &mut Vec<char>) {
        match self {
            Self::SwapPositions(x, y) => {
                let t = chars[x];
                chars[x] = chars[y];
                chars[y] = t
            }
            Self::SwapLetters(x, y) => {
                let x = chars.iter().position(|&c| c == x).unwrap();
                let y = chars.iter().position(|&c| c == y).unwrap();
                Self::SwapPositions(x, y).apply(chars);
            }
            Self::RotateLeft(x) => chars.rotate_left(x),
            Self::RotateRight(x) => chars.rotate_right(x),
            Self::RotateBasedOn(x) => {
                let i = chars.iter().position(|&c| c == x).unwrap();
                chars.rotate_right(i + 1);
                if i >= 4 {
                    chars.rotate_right(1);
                }
            }
            Self::Reverse(l, r) => chars[l..=r].reverse(),
            Self::Move(x, y) => {
                let c = chars.remove(x);
                chars.insert(y, c);
            }
        }
    }

    fn undo(self, chars: &mut Vec<char>) {
        match self {
            Self::SwapPositions(_, _) | Self::SwapLetters(_, _) | Self::Reverse(_, _) => {
                self.apply(chars)
            }

            Self::RotateLeft(x) => chars.rotate_right(x),
            Self::RotateRight(x) => chars.rotate_left(x),
            Self::RotateBasedOn(_) => {
                let mut i = 0;
                loop {
                    let mut chars_2 = chars.clone();
                    chars_2.rotate_left(i);
                    self.apply(&mut chars_2);
                    if chars_2 == *chars {
                        chars.rotate_left(i);
                        break;
                    }
                    i += 1
                }
            }
            Self::Move(x, y) => Self::Move(y, x).apply(chars),
        }
    }

    fn parse_swap_positions(input: &mut &str) -> Result<Self> {
        "swap position ".parse_next(input)?;
        let x: usize = dec_uint(input)?;
        " with position ".parse_next(input)?;
        let y: usize = dec_uint(input)?;

        Ok(Self::SwapPositions(x, y))
    }

    fn parse_swap_letters(input: &mut &str) -> Result<Self> {
        "swap letter ".parse_next(input)?;
        let x = any(input)?;
        " with letter ".parse_next(input)?;
        let y = any(input)?;
        Ok(Self::SwapLetters(x, y))
    }

    fn parse_rotate_left(input: &mut &str) -> Result<Self> {
        "rotate left ".parse_next(input)?;
        let x: usize = dec_uint(input)?;
        " step".parse_next(input)?;
        opt('s').parse_next(input)?;
        Ok(Self::RotateLeft(x))
    }

    fn parse_rotate_right(input: &mut &str) -> Result<Self> {
        "rotate right ".parse_next(input)?;
        let x: usize = dec_uint(input)?;
        " step".parse_next(input)?;
        opt('s').parse_next(input)?;
        Ok(Self::RotateRight(x))
    }

    fn parse_rotate_based_on(input: &mut &str) -> Result<Self> {
        "rotate based on position of letter ".parse_next(input)?;
        let x = any(input)?;
        Ok(Self::RotateBasedOn(x))
    }

    fn parse_reverse(input: &mut &str) -> Result<Self> {
        "reverse positions ".parse_next(input)?;
        let x: usize = dec_uint(input)?;
        " through ".parse_next(input)?;
        let y: usize = dec_uint(input)?;
        Ok(Self::Reverse(x, y))
    }

    fn parse_move(input: &mut &str) -> Result<Self> {
        "move position ".parse_next(input)?;
        let x: usize = dec_uint(input)?;
        " to position ".parse_next(input)?;
        let y: usize = dec_uint(input)?;
        Ok(Self::Move(x, y))
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((
            Self::parse_swap_letters,
            Self::parse_swap_positions,
            Self::parse_rotate_left,
            Self::parse_rotate_right,
            Self::parse_rotate_based_on,
            Self::parse_reverse,
            Self::parse_move,
        ))
        .parse_next(input)
    }
}

fn solve(input: &str) -> (String, String) {
    let mut chars: Vec<char> = "abcdefgh".chars().collect();
    for line in input.lines() {
        let instruction = Instruction::parse.parse(line).unwrap();
        instruction.apply(&mut chars);
    }
    let output_1 = chars.iter().collect();
    let mut chars: Vec<char> = "fbgdceah".chars().collect();
    for line in input.lines().rev() {
        let instruction = Instruction::parse.parse(line).unwrap();
        instruction.undo(&mut chars);
    }
    let output_2 = chars.iter().collect();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
