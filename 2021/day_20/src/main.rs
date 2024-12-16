use std::{collections::HashSet, fs::read_to_string};

struct EnhanceMap([bool; 512]);

impl EnhanceMap {
    fn parse(input: &str) -> Self {
        Self(
            input
                .chars()
                .filter(|&c| c == '.' || c == '#')
                .map(|c| c == '#')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

#[derive(Debug)]
struct Image {
    positions: HashSet<Position>,
    bg_state: bool,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Image {
    fn parse(input: &str) -> Self {
        let positions = input
            .lines()
            .enumerate()
            .map(|(i, line)| (i, line.char_indices()))
            .flat_map(|(y, chars)| chars.map(move |x| (y, x)))
            .filter(|&(_, (_, c))| c == '#')
            .map(|(y, (x, _))| Position {
                x: x as i32,
                y: y as i32,
            })
            .collect::<HashSet<_>>();

        let bg_state = false;
        let min_x = positions.iter().map(|p| p.x).min().unwrap();
        let max_x = positions.iter().map(|p| p.x).max().unwrap();
        let min_y = positions.iter().map(|p| p.y).min().unwrap();
        let max_y = positions.iter().map(|p| p.y).max().unwrap();

        Self {
            positions,
            bg_state,
            max_y,
            min_y,
            max_x,
            min_x,
        }
    }

    fn enhance(self, enhance_map: &EnhanceMap) -> Self {
        let min_x = self.min_x - 1;
        let max_x = self.max_x + 1;
        let min_y = self.min_y - 1;
        let max_y = self.max_y + 1;

        let positions = (min_x..=max_x)
            .flat_map(|x| (min_y..=max_y).map(move |y| (x, y)))
            .map(|(x, y)| Position { x, y })
            .filter(|&position| {
                let bits = self.get_bits(position);
                let index = bits.to_usize();
                enhance_map.0[index]
            })
            .collect();

        let bg_state = if self.bg_state {
            enhance_map.0[511]
        } else {
            enhance_map.0[0]
        };

        Self {
            bg_state,
            positions,
            max_y,
            min_y,
            max_x,
            min_x,
        }
    }

    fn get_bits(&self, position: Position) -> Bits {
        let min_x = position.x - 1;
        let max_x = position.x + 1;
        let min_y = position.y - 1;
        let max_y = position.y + 1;
        Bits(
            (min_y..=max_y)
                .flat_map(|y| (min_x..=max_x).map(move |x| (x, y)))
                .map(|(x, y)| Position { x, y })
                .map(|p| self.get_bit(p))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    fn get_bit(&self, position: Position) -> bool {
        if self.is_in_range(position) {
            self.positions.contains(&position)
        } else {
            self.bg_state
        }
    }

    fn is_in_range(&self, position: Position) -> bool {
        self.min_x <= position.x
            && position.x <= self.max_x
            && self.min_y <= position.y
            && position.y <= self.max_y
    }
}

struct Bits([bool; 9]);
impl Bits {
    fn to_usize(&self) -> usize {
        self.0
            .iter()
            .fold(0, |acc, &b| acc * 2 + b as usize)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let mut split = input.split("\n\n");
    let enhance_map = EnhanceMap::parse(split.next().unwrap());
    let mut image = Image::parse(split.next().unwrap());

    for _ in 0..2 {
        image = image.enhance(&enhance_map);
    }

    let output_1 = image.positions.len();
    for _ in 0..48 {
        image = image.enhance(&enhance_map);
    }
    let output_2 = image.positions.len();

    (output_1, output_2)
}

// 5278 too high
fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
