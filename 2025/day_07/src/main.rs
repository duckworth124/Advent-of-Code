use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

struct Grid {
    tiles: Vec<Vec<bool>>,
    start: (usize, usize),
}

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|l| l.chars().map(|c| c == '^').collect())
            .collect();
        let start = (input.find('S').unwrap(), 0);
        Self { tiles, start }
    }

    fn count_splits(&self) -> u32 {
        let height = self.tiles.len();
        let width = self.tiles[0].len();
        let mut frontier = vec![self.start];
        let mut visited = HashSet::new();
        let mut count = 0;
        while let Some(current) = frontier.pop() {
            let (x, y) = current;
            if !visited.insert(current) {
                continue;
            }
            if y == height {
                continue;
            }
            if x == width {
                continue;
            }
            if self.tiles[y][x] {
                frontier.push((x.saturating_sub(1), y));
                frontier.push((x + 1, y));
                count += 1;
                continue;
            }
            frontier.push((x, y + 1));
        }
        count
    }

    fn quantum_splits(
        &self,
        start: (usize, usize),
        cache: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        let mut current = start;
        let height = self.tiles.len();
        if let Some(&output) = cache.get(&start) {
            return output;
        }
        loop {
            current.1 += 1;
            let (x, y) = current;
            if y == height {
                return 1;
            }
            if self.tiles[y][x] {
                let output =
                    self.quantum_splits((x - 1, y), cache) + self.quantum_splits((x + 1, y), cache);
                cache.insert(start, output);
                return output;
            }
        }
    }
}

fn solve(input: &str) -> (u32, u64) {
    let grid = Grid::parse(input);
    (
        grid.count_splits(),
        grid.quantum_splits(grid.start, &mut HashMap::new()),
    )
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
