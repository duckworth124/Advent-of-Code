use itertools::iproduct;
use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

enum Type {
    Rocky,
    Narrow,
    Wet,
}
impl Type {
    const fn risk_level(self) -> u32 {
        match self {
            Self::Rocky => 0,
            Self::Narrow => 2,
            Self::Wet => 1,
        }
    }
}

fn calculate_geologic_index(
    position: (u32, u32),
    target: (u32, u32),
    cave_depth: u32,
    cache: &mut HashMap<(u32, u32), u32>,
) -> u32 {
    if let Some(&output) = cache.get(&position) {
        return output;
    }
    if position == (0, 0) || position == target {
        return 0;
    }
    let (x, y) = position;
    if y == 0 {
        return (x * 16807) % 20183;
    }
    if x == 0 {
        return (y * 48271) % 20183;
    }
    let output = (calculate_erosion_level((x - 1, y), target, cave_depth, cache)
        * calculate_erosion_level((x, y - 1), target, cave_depth, cache))
        % 20183;
    cache.insert(position, output);
    output
}

fn calculate_erosion_level(
    position: (u32, u32),
    target: (u32, u32),
    cave_depth: u32,
    cache: &mut HashMap<(u32, u32), u32>,
) -> u32 {
    (calculate_geologic_index(position, target, cave_depth, cache) + cave_depth) % 20183
}

fn calculate_type(
    position: (u32, u32),
    target: (u32, u32),
    cave_depth: u32,
    cache: &mut HashMap<(u32, u32), u32>,
) -> Type {
    let erosion_level = calculate_erosion_level(position, target, cave_depth, cache);
    match erosion_level % 3 {
        0 => Type::Rocky,
        1 => Type::Wet,
        _ => Type::Narrow,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Equipment {
    Torch,
    ClimbingGear,
    Neither,
}

impl Equipment {
    const fn all() -> [Self; 3] {
        [Self::Torch, Self::ClimbingGear, Self::Neither]
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    fn step(self, position: (u32, u32)) -> Option<(u32, u32)> {
        let (x, y) = position;
        Some(match self {
            Self::Up => (x, y.checked_sub(1)?),
            Self::Down => (x, y + 1),
            Self::Left => (x.checked_sub(1)?, y),
            Self::Right => (x + 1, y),
        })
    }
}

#[derive(Clone, Copy)]
enum Action {
    Move(Direction),
    ChangeEquipment(Equipment),
}

impl Action {
    fn all() -> Vec<Self> {
        Direction::all()
            .into_iter()
            .map(Self::Move)
            .chain(Equipment::all().map(Self::ChangeEquipment))
            .collect()
    }

    const fn cost(self) -> u32 {
        match self {
            Self::Move(_) => 1,
            Self::ChangeEquipment(_) => 7,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: (u32, u32),
    equipment: Equipment,
}

impl State {
    fn apply_action(
        self,
        action: Action,
        target: (u32, u32),
        cave_depth: u32,
        cache: &mut HashMap<(u32, u32), u32>,
    ) -> Option<Self> {
        let new = match action {
            Action::Move(direction) => {
                let new_position = direction.step(self.position)?;
                Self {
                    position: new_position,
                    ..self
                }
            }
            Action::ChangeEquipment(equipment) => Self { equipment, ..self },
        };

        Some(new).filter(|new| new.is_valid(target, cave_depth, cache))
    }

    fn is_valid(
        self,
        target: (u32, u32),
        cave_depth: u32,
        cache: &mut HashMap<(u32, u32), u32>,
    ) -> bool {
        let current_type = calculate_type(self.position, target, cave_depth, cache);
        match current_type {
            Type::Rocky => self.equipment != Equipment::Neither,
            Type::Narrow => self.equipment != Equipment::ClimbingGear,
            Type::Wet => self.equipment != Equipment::Torch,
        }
    }

    fn is_goal(self, target: (u32, u32)) -> bool {
        self.position == target && matches!(self.equipment, Equipment::Torch)
    }
}

fn shortest_time(target: (u32, u32), cave_depth: u32, cache: &mut HashMap<(u32, u32), u32>) -> u32 {
    let start = State {
        position: (0, 0),
        equipment: Equipment::Torch,
    };
    let mut frontier: PriorityQueue<State, Reverse<u32>> =
        PriorityQueue::from_iter([(start, Reverse(0))]);
    let mut seen = HashSet::new();
    loop {
        let (current, Reverse(cost)) = frontier.pop().unwrap();
        if current.is_goal(target) {
            return cost;
        }
        if !seen.insert(current) {
            continue;
        }
        Action::all()
            .into_iter()
            .filter_map(|a| {
                current
                    .apply_action(a, target, cave_depth, cache)
                    .zip(Some(a.cost()))
            })
            .for_each(|(s, c)| {
                frontier.push_increase(s, Reverse(cost + c));
            });
    }
}

fn solve(input: &str) -> (u32, u32) {
    let cave_depth: u32 = input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("depth: ")
        .unwrap()
        .parse()
        .unwrap();
    let target = input
        .lines()
        .nth(1)
        .unwrap()
        .strip_prefix("target: ")
        .unwrap()
        .split_once(',')
        .unwrap();
    let target: (u32, u32) = (target.0.parse().unwrap(), target.1.parse().unwrap());
    let mut cache = HashMap::new();
    let output_1 = iproduct!(0..=target.0, 0..=target.1)
        .map(|p| calculate_type(p, target, cave_depth, &mut cache))
        .map(|t| t.risk_level())
        .sum();

    let output_2 = shortest_time(target, cave_depth, &mut cache);
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
