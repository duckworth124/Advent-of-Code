use std::fs::read_to_string;

use itertools::iproduct;

#[derive(Clone, Copy)]
enum Alignment {
    Elf,
    Goblin,
}

impl Alignment {
    fn opposite(self) -> Self {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct Agent {
    hp: u32,
    alignment: Alignment,
}

impl Agent {
    const fn new(alignment: Alignment) -> Self {
        Self { hp: 200, alignment }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Occupied(Agent),
}

impl Tile {
    const fn get_agent(self) -> Option<Agent> {
        match self {
            Tile::Occupied(agent) => Some(agent),
            _ => None,
        }
    }
}

struct State {
    grid: Vec<Vec<Tile>>,
    time: u32,
}

impl State {
    fn parse(input: &str) -> Self {
        let grid: Vec<Vec<Tile>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => Tile::Wall,
                        '.' => Tile::Empty,
                        'E' => Tile::Occupied(Agent::new(Alignment::Elf)),
                        'G' => Tile::Occupied(Agent::new(Alignment::Goblin)),
                        _ => panic!("unrecognized character: {c:?}"),
                    })
                    .collect()
            })
            .collect();

        let time = 0;
        Self { grid, time }
    }

    fn update(&mut self, position: (usize, usize)) -> bool {
        let (x, y) = position
        let agent = if let Some(x) = self.grid[y][x].get_agent() {
            x
        } else {
            return true;
        };

        let target_alignment = agent.alignment.opposite();
        let all_targets = self.find_all_targets(target_alignment);
        let (nearest_target, distance) = all_targets.iter().copied().filter_map(|p| 
            Some(p).zip(self.distance(position, p))
            )
            .min_by_key(|((x, y), d)| (d, y, x))
            .unwrap();
    }

    fn distance(&self, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
        todo!()
    }

    fn find_all_targets(&self, target_alignment: Alignment) -> Vec<(usize, usize)> {
        todo!()
    }

    fn step(&mut self) -> bool {
        let width = self.grid[0].len();
        let height = self.grid.len();
        iproduct!(0..width, 0..height).all(|c| self.update(c))
    }

    fn total_hp(&self) -> u32 {
        self.grid
            .iter()
            .flatten()
            .filter_map(|t| t.get_agent())
            .map(|a| a.hp)
            .sum()
    }
}

fn solve(input: &str) -> u32 {
    let mut state = State::parse(input);
    while state.step() {}
    let output_1 = state.time * state.total_hp();
    output_1
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = solve(&input);
    println!("part 1: {output_1}")
}
