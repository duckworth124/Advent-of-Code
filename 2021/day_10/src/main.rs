use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum BracketDirection {
    Open,
    Close,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum BracketType {
    Paren,
    Bracket,
    Brace,
    Chevron,
}

impl BracketType {
    fn incomplete_score(&self) -> u64 {
        match self {
            BracketType::Paren => 1,
            BracketType::Bracket => 2,
            BracketType::Brace => 3,
            BracketType::Chevron => 4,
        }
    }
}

#[derive(Clone, Copy)]
struct Token {
    direction: BracketDirection,
    bracket_type: BracketType,
}

impl Token {
    fn new(c: char) -> Self {
        let direction = if matches!(c, '(' | '[' | '{' | '<') {
            BracketDirection::Open
        } else {
            BracketDirection::Close
        };

        let bracket_type = match c {
            '(' | ')' => BracketType::Paren,
            '[' | ']' => BracketType::Bracket,
            '{' | '}' => BracketType::Brace,
            '<' | '>' => BracketType::Chevron,
            _ => panic!("invalid character"),
        };

        Self {
            direction,
            bracket_type,
        }
    }

    fn is_open(&self) -> bool {
        matches!(self.direction, BracketDirection::Open)
    }

    fn corrupted_score(&self) -> u32 {
        match self.bracket_type {
            BracketType::Paren => 3,
            BracketType::Bracket => 57,
            BracketType::Brace => 1197,
            BracketType::Chevron => 25137,
        }
    }
}

struct Line(Vec<Token>);

impl Line {
    fn new(input: &str) -> Self {
        let tokens = input.chars().map(Token::new).collect();
        Self(tokens)
    }

    fn first_invalid_token(&self) -> Option<Token> {
        let mut unclosed_tokens = vec![];
        for token in self.0.iter() {
            if token.is_open() {
                unclosed_tokens.push(token.bracket_type);
            } else if unclosed_tokens.pop() != Some(token.bracket_type) {
                return Some(*token);
            }
        }

        None
    }

    fn corrupted_score(&self) -> Option<u32> {
        let first_invalid_token = self.first_invalid_token()?;
        Some(first_invalid_token.corrupted_score())
    }

    fn incomplete_score(&self) -> Option<u64> {
        let mut unclosed_tokens = vec![];
        for token in self.0.iter() {
            if token.is_open() {
                unclosed_tokens.push(token.bracket_type)
            } else if unclosed_tokens.pop() != Some(token.bracket_type) {
                return None;
            }
        }

        unclosed_tokens.reverse();
        let mut total = 0;
        for bracket_type in unclosed_tokens {
            total *= 5;
            total += bracket_type.incomplete_score();
        }

        Some(total)
    }
}

struct Lines(Vec<Line>);

impl Lines {
    fn new(input: &str) -> Self {
        let lines = input.lines().map(Line::new).collect();
        Lines(lines)
    }

    fn total_corrupted_score(&self) -> u32 {
        self.0.iter().flat_map(|l| l.corrupted_score()).sum()
    }

    fn incomplete_scores(&self) -> Vec<u64> {
        self.0.iter().flat_map(|l| l.incomplete_score()).collect()
    }

    fn middle_incomplete_score(&self) -> u64 {
        let mut incomplete_scores = self.incomplete_scores();
        incomplete_scores.sort();
        let middle = (incomplete_scores.len() - 1) / 2;
        incomplete_scores[middle]
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let lines = Lines::new(&input);
    let output_1 = lines.total_corrupted_score();
    let output_2 = lines.middle_incomplete_score();

    println!("part 1: {output_1} part 2: {output_2}")
}
