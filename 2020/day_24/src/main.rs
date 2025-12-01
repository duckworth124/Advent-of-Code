use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct HexPosition {
    right: i32,
    up_right: i32,
}

impl HexPosition {
    fn step(self, direction: &str) -> Self {
        let (right, up_right) = match direction {
            "e" => (self.right + 1, self.up_right),
            "w" => (self.right - 1, self.up_right),
            "ne" => (self.right, self.up_right + 1),
            "sw" => (self.right, self.up_right - 1),
            "se" => (self.right + 1, self.up_right - 1),
            "nw" => (self.right - 1, self.up_right + 1),
            _ => panic!("unrecognized direction {direction:?}"),
        };

        Self { right, up_right }
    }

    fn adjecent(self) -> [Self; 6] {
        [(1, 0), (0, 1), (-1, 0), (0, -1), (1, -1), (-1, 1)]
            .map(|(x, y)| (self.right + x, self.up_right + y))
            .map(|(right, up_right)| Self { right, up_right })
    }
}

fn next_conway(prev: HashSet<HexPosition>) -> HashSet<HexPosition> {
    prev.iter()
        .copied()
        .flat_map(|p| p.adjecent())
        .filter(|p| {
            let surrounding = p.adjecent();
            let neighbour_count = surrounding.into_iter().filter(|p| prev.contains(p)).count();
            if neighbour_count > 2 {
                return false;
            }
            if neighbour_count == 0 {
                return false;
            }
            if neighbour_count == 2 {
                return true;
            }
            prev.contains(p)
        })
        .collect()
}

fn solve(input: &str) -> (usize, usize) {
    let mut black_tiles = HashSet::new();
    for mut line in input.lines() {
        let mut pos = HexPosition {
            right: 0,
            up_right: 0,
        };
        while !line.is_empty() {
            let c = line.chars().next().unwrap();
            if c == 'e' {
                pos = pos.step("e");
                line = &line[1..];
                continue;
            }
            if c == 'w' {
                pos = pos.step("w");
                line = &line[1..];
                continue;
            }
            pos = pos.step(&line[..2]);
            line = &line[2..];
        }
        if !black_tiles.insert(pos) {
            black_tiles.remove(&pos);
        }
    }

    let output_1 = black_tiles.len();
    for _ in 0..100 {
        black_tiles = next_conway(black_tiles);
    }
    let output_2 = black_tiles.len();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
