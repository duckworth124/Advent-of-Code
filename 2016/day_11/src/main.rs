use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum DeviceType {
    Microchip,
    Generator,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Device {
    name: String,
    device_type: DeviceType,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    floors: [Vec<Device>; 4],
    current_floor: usize,
}

impl State {
    fn is_valid(&self) -> bool {
        for floor in &self.floors {
            for device in floor {
                if device.device_type == DeviceType::Microchip {
                    if floor
                        .iter()
                        .any(|d| d.name == device.name && d.device_type == DeviceType::Generator)
                    {
                        continue;
                    }

                    if floor.iter().any(|d| d.device_type == DeviceType::Generator) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn apply_action(&self, action: Action) -> Option<Self> {
        let mut output = self.clone();
        if self.current_floor == 0 && action.direction == Direction::Down {
            return None;
        }
        if self.current_floor == 3 && action.direction == Direction::Up {
            return None;
        }
        for device in &action.holding {
            output.floors[self.current_floor].retain(|d| d != device);
        }
        let new_floor = match action.direction {
            Direction::Down => self.current_floor.saturating_sub(1),
            Direction::Up => self.current_floor + 1,
        };
        output.floors[new_floor].extend(action.holding);
        output.current_floor = new_floor;
        if !output.is_valid() {
            return None;
        }

        Some(output)
    }

    fn possible_actions(&self) -> Vec<Action> {
        self.floors[self.current_floor]
            .iter()
            .combinations(2)
            .chain(self.floors[self.current_floor].iter().map(|d| vec![d]))
            .cartesian_product(&[Direction::Up, Direction::Down])
            .map(|(v, d)| Action {
                holding: v.iter().cloned().cloned().collect(),
                direction: *d,
            })
            .collect()
    }

    fn min_steps(&self) -> Option<u32> {
        let mut frontier = VecDeque::from([(self.clone(), 0)]);
        let mut visited = HashSet::new();
        loop {
            let (current_state, time) = frontier.pop_front()?;
            if current_state.floors.iter().take(3).all(|v| v.is_empty()) {
                return Some(time);
            }
            if !visited.insert(current_state.clone()) {
                continue;
            }
            current_state
                .possible_actions()
                .into_iter()
                .filter_map(|a| current_state.apply_action(a))
                .for_each(|s| frontier.push_back((s, time + 1)));
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
struct Action {
    holding: Vec<Device>,
    direction: Direction,
}

fn parse_floor(line: &str) -> Vec<Device> {
    let line = line.split_once("contains").unwrap().1;
    let line = line.replace("and", ",");
    line.split(',')
        .filter(|s| s != &" ")
        .map(|s| {
            if s.contains("microchip") {
                let name = s.split_once(" a ").unwrap().1.split_once('-').unwrap().0;
                Device {
                    name: name.to_string(),
                    device_type: DeviceType::Microchip,
                }
            } else {
                let name = s
                    .split_once(" a ")
                    .unwrap()
                    .1
                    .split_once(" generator")
                    .unwrap()
                    .0;
                Device {
                    name: name.to_string(),
                    device_type: DeviceType::Generator,
                }
            }
        })
        .collect()
}

fn solve(input: &str) -> u32 {
    let floors: [Vec<Device>; 4] = input
        .lines()
        .take(3)
        .map(parse_floor)
        .chain([vec![]])
        .collect_vec()
        .try_into()
        .unwrap();

    let state = State {
        floors,
        current_floor: 0,
    };

    state.min_steps().unwrap()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = solve(&input);
    println!("part 1: {output_1}")
}
