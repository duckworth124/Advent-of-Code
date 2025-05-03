use itertools::Itertools;
use std::io::Write;
use std::{
    cmp::Reverse,
    fs::{File, read_to_string},
};
use winnow::{
    Parser, Result,
    ascii::{alpha1, dec_uint, newline},
    combinator::{separated, seq},
    token::take_while,
};

#[derive(Debug)]
struct Room {
    name: String,
    id: u32,
    checksum: Vec<char>,
}

impl Room {
    fn is_valid(&self) -> bool {
        let chars = self
            .name
            .chars()
            .filter(|c| c.is_alphabetic())
            .counts()
            .into_iter()
            .sorted_by_key(|(c, _)| *c)
            .sorted_by_key(|(_, i)| Reverse(*i))
            .take(5)
            .map(|(c, _)| c)
            .collect::<Vec<char>>();
        chars == self.checksum
    }

    fn parse(input: &mut &str) -> Result<Self> {
        seq! {
            Self {
                name: take_while(0.., |c: char| !c.is_ascii_digit()).map(|s: &str| s.to_owned()),
                id: dec_uint,
                _ : '[',
                checksum: alpha1.map(|s: &str| s.chars().collect()),
                _ : ']'


            }
        }
        .parse_next(input)
    }

    fn decrypt(&self) -> String {
        self.name
            .split('-')
            .map(|s| {
                s.chars()
                    .map(|c| c as u32 - 'a' as u32)
                    .map(|x| (x + self.id) % 26)
                    .map(|x| (x + 'a' as u32) as u8 as char)
                    .collect::<String>()
            })
            .join(" ")
    }
}

fn solve(mut input: &str) -> (u32, u32) {
    let rooms: Vec<Room> = separated(1.., Room::parse, newline)
        .parse_next(&mut input)
        .unwrap();
    let output_1 = rooms.iter().filter(|r| r.is_valid()).map(|r| r.id).sum();
    let mut f = File::create("rooms").unwrap();
    for room in &rooms {
        write!(f, "{}-{}", room.decrypt(), room.id).unwrap();
        writeln!(f).unwrap();
    }
    (output_1, 548)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
