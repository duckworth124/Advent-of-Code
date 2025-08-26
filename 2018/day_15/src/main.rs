use itertools::{Itertools, iproduct};
use std::{
    collections::{HashSet, VecDeque},
    fmt::{Debug, Display},
    fs::read_to_string,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Alignment {
    Elf,
    Goblin,
}

impl Alignment {
    const fn opposite(self) -> Self {
        match self {
            Self::Elf => Self::Goblin,
            Self::Goblin => Self::Elf,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Agent {
    hp: u32,
    alignment: Alignment,
    can_act: bool,
    attack_power: u32,
}

impl Agent {
    const fn new(alignment: Alignment) -> Self {
        Self {
            hp: 200,
            alignment,
            can_act: true,
            attack_power: 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Wall,
    Empty,
    Occupied(Agent),
}

impl Tile {
    const fn get_agent(self) -> Option<Agent> {
        match self {
            Self::Occupied(agent) => Some(agent),
            _ => None,
        }
    }
}

struct State {
    grid: Vec<Vec<Tile>>,
    time: u32,
}

impl State {
    fn surrounding(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let width = self.grid[0].len();
        let height = self.grid.len();
        let (x, y) = position;
        [
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
            (x + 1, y),
            (x, y + 1),
        ]
        .into_iter()
        .filter(|&p| p != position)
        .filter(|&(x, y)| x < width && y < height)
        .filter(|&p| !self.is_blocked(p))
        .collect()
    }

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
        let width = self.grid[0].len();
        let height = self.grid.len();
        let (x, y) = position;
        let agent = if let Some(x) = self.grid[y][x].get_agent() {
            x
        } else {
            return true;
        };
        if !agent.can_act {
            return true;
        }

        let target_alignment = agent.alignment.opposite();
        let all_targets = self.find_all_targets(target_alignment);
        if all_targets.is_empty() {
            return false;
        }

        let destination = if [
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
            (x + 1, y),
            (x, y + 1),
        ]
        .into_iter()
        .filter(|&p| p != position)
        .filter(|&(x, y)| x < width && y < height)
        .map(|(x, y)| (self.grid[y][x]))
        .any(|t| matches!(t, Tile::Occupied(a) if a.alignment == agent.alignment.opposite()))
        {
            position
        } else {
            let (_, _, (_, destination)) = match all_targets
                .iter()
                .copied()
                .flat_map(|target| {
                    self.surrounding(target)
                        .into_iter()
                        .map(move |in_range| (target, in_range))
                })
                .filter_map(|(target, in_range)| {
                    Some((target, in_range, self.pathfind(position, in_range)?))
                })
                .min_by_key(|&(_, in_range, (d, next))| (d, in_range.1, in_range.0, next.1, next.0))
            {
                Some(x) => x,
                None => {
                    return true;
                }
            };
            destination
        };

        self.grid[y][x] = Tile::Empty;
        let (x, y) = destination;
        self.grid[y][x] = Tile::Occupied(Agent {
            can_act: false,
            ..agent
        });
        let nearest_target = [
            (x, y.saturating_sub(1)),
            (x.saturating_sub(1), y),
            (x + 1, y),
            (x, y + 1),
        ]
        .into_iter()
        .filter(|&p| p != position)
        .filter(|&(x, y)| x < width && y < height)
        .filter(|&(x, y)| matches!(self.grid[y][x], Tile::Occupied(a) if a.alignment == agent.alignment.opposite()))
        .min_by_key(|&(x, y)| self.grid[y][x].get_agent().unwrap().
            hp);

        if let Some(nearest_target) = nearest_target
            && is_adjacent(destination, nearest_target)
        {
            self.deal_damage(nearest_target, agent.attack_power);
        }

        true
    }

    fn deal_damage(&mut self, position: (usize, usize), damage: u32) {
        let (x, y) = position;
        if let Tile::Occupied(agent) = &mut self.grid[y][x] {
            agent.hp = agent.hp.saturating_sub(damage);
            if agent.hp == 0 {
                self.grid[y][x] = Tile::Empty
            }
        }
    }

    fn pathfind(
        &self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Option<(u32, (usize, usize))> {
        let distance = self.distance(start, end)?;
        let next = if distance == 0 {
            start
        } else {
            self.surrounding(start)
                .into_iter()
                .filter(|&p| self.distance(p, end) == Some(distance - 1))
                .sorted_unstable_by_key(|&(x, y)| (y, x))
                .next()
                .unwrap()
        };

        Some((distance, next))
    }

    fn distance(&self, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
        let mut frontier = VecDeque::from([(start, 0)]);
        let mut visited = HashSet::new();
        let width = self.grid[0].len();
        let height = self.grid.len();
        loop {
            let (current_position, current_cost) = frontier.pop_front()?;
            if !visited.insert(current_position) {
                continue;
            }
            if current_position == end {
                return Some(current_cost);
            }
            if current_position != start && self.is_blocked(current_position) {
                continue;
            }
            let (x, y) = current_position;
            frontier.extend(
                [
                    (x.saturating_sub(1), y),
                    (x, y.saturating_sub(1)),
                    (x + 1, y),
                    (x, y + 1),
                ]
                .into_iter()
                .filter(|&(x, y)| x < width && y < height)
                .map(|p| (p, current_cost + 1)),
            );
        }
    }

    fn find_all_targets(&self, target_alignment: Alignment) -> Vec<(usize, usize)> {
        let width = self.grid[0].len();
        let height = self.grid.len();

        iproduct!(0..height,0..width)
            .map(|(x,y)|(y,x))
            .filter(|&(x, y)| matches!(self.grid[y][x], Tile::Occupied(a) if a.alignment == target_alignment))
            .collect()
    }

    fn step(&mut self) -> bool {
        let width = self.grid[0].len();
        let height = self.grid.len();
        let output = iproduct!(0..height, 0..width).all(|(y, x)| self.update((x, y)));
        if output {
            self.time += 1;
            self.grid.iter_mut().flatten().for_each(|t| {
                if let Tile::Occupied(a) = t {
                    a.can_act = true
                }
            });
        }
        output
    }

    fn is_blocked(&self, position: (usize, usize)) -> bool {
        let (x, y) = position;
        !matches!(self.grid[y][x], Tile::Empty)
    }

    fn total_hp(&self) -> u32 {
        self.grid
            .iter()
            .flatten()
            .filter_map(|t| t.get_agent())
            .map(|a| a.hp)
            .sum()
    }

    fn will_elves_win_without_casualties(&mut self) -> bool {
        let elf_count = self
            .grid
            .iter()
            .flatten()
            .filter_map(|t| t.get_agent())
            .filter(|a| a.alignment == Alignment::Elf)
            .count();

        while self.step() {
            let new_elf_count = self
                .grid
                .iter()
                .flatten()
                .filter_map(|t| t.get_agent())
                .filter(|a| a.alignment == Alignment::Elf)
                .count();

            if new_elf_count < elf_count {
                return false;
            }
        }

        let new_elf_count = self
            .grid
            .iter()
            .flatten()
            .filter_map(|t| t.get_agent())
            .filter(|a| a.alignment == Alignment::Elf)
            .count();

        new_elf_count == elf_count
    }

    fn set_elf_attack(&mut self, attack: u32) {
        self.grid.iter_mut().flatten().for_each(|t| match t {
            Tile::Occupied(a) if a.alignment == Alignment::Elf => a.attack_power = attack,
            _ => {}
        })
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for t in row {
                match t {
                    Tile::Wall => write!(f, "#")?,
                    Tile::Empty => write!(f, ".")?,
                    Tile::Occupied(agent) => match agent.alignment {
                        Alignment::Elf => write!(f, "E")?,
                        Alignment::Goblin => write!(f, "G")?,
                    },
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

const fn is_adjacent(p1: (usize, usize), p2: (usize, usize)) -> bool {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) == 1
}

fn solve(input: &str) -> (u32, u32) {
    let mut state = State::parse(input);
    while state.step() {}
    let output_1 = state.time * state.total_hp();
    let min_attack = {
        let mut low = 3;
        let mut high = 200;
        while high - 1 > low {
            let mid = low + (high - low) / 2;
            dbg!(mid);
            let mut state = State::parse(input);
            state.set_elf_attack(mid);
            if state.will_elves_win_without_casualties() {
                high = mid;
            } else {
                low = mid
            }
        }
        high
    };
    let mut state = State::parse(input);
    state.set_elf_attack(min_attack);
    while state.step() {}
    let output_2 = state.time * state.total_hp();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
