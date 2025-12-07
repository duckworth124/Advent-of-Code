use std::{fs::read_to_string, time::Instant};

struct Grid<'a> {
    tiles: &'a str,
    start: usize,
    width: usize,
}

impl<'a> Grid<'a> {
    fn parse(input: &'a str) -> Self {
        let tiles = input;
        let width = input.find('\n').unwrap();
        let start = input.find('S').unwrap();
        Self {
            tiles,
            start,
            width,
        }
    }

    fn splits(&self) -> (u32, u64) {
        let mut occupied = vec![0; self.width];
        occupied[self.start] = 1;
        let mut splits = 0;
        for row in self.tiles.lines() {
            for (x, c) in row.char_indices() {
                if c == '^' && occupied[x] > 0 {
                    occupied[x - 1] += occupied[x];
                    occupied[x + 1] += occupied[x];
                    occupied[x] = 0;
                    splits += 1
                }
            }
        }
        (splits, occupied.into_iter().sum())
    }
}

fn solve(input: &str) -> (u32, u64) {
    let grid = Grid::parse(input);
    grid.splits()
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
