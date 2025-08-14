use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashSet, fs::read_to_string};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum AmphipodVariant {
    A,
    B,
    C,
    D,
}

impl AmphipodVariant {
    const fn step_cost(self) -> u32 {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CorridoorLocation {
    FarLeft,
    LeftA,
    AB,
    BC,
    CD,
    RightD,
    FarRight,
}

impl CorridoorLocation {
    fn all() -> Vec<Self> {
        [
            Self::FarLeft,
            Self::LeftA,
            Self::AB,
            Self::BC,
            Self::CD,
            Self::RightD,
            Self::FarRight,
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Location {
    Room(AmphipodVariant, u32),
    Corridoor(CorridoorLocation),
}

impl Location {
    const fn is_corridoor(self) -> bool {
        matches!(self, Self::Corridoor(_))
    }

    const fn is_room(self) -> bool {
        matches!(self, Self::Room(_, _))
    }

    const fn to_coords(self) -> (u32, u32) {
        match self {
            Self::Room(amphipod_variant, position_in_room) => {
                let x = match amphipod_variant {
                    AmphipodVariant::A => 2,
                    AmphipodVariant::B => 4,
                    AmphipodVariant::C => 6,
                    AmphipodVariant::D => 8,
                };
                let y = position_in_room + 1;

                (x, y)
            }
            Self::Corridoor(corridoor_location) => {
                let x = match corridoor_location {
                    CorridoorLocation::FarLeft => 0,
                    CorridoorLocation::LeftA => 1,
                    CorridoorLocation::AB => 3,
                    CorridoorLocation::BC => 5,
                    CorridoorLocation::CD => 7,
                    CorridoorLocation::RightD => 9,
                    CorridoorLocation::FarRight => 10,
                };

                (x, 0)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Amphipod {
    variant: AmphipodVariant,
    location: Location,
    can_move: bool,
}

impl Amphipod {
    fn is_in_correct_room(self) -> bool {
        if let Location::Room(r, _) = self.location {
            r == self.variant
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Movement {
    ToCorridoor(CorridoorLocation),
    ToRoom,
}

impl Movement {
    fn all() -> Vec<Self> {
        CorridoorLocation::all()
            .into_iter()
            .map(Self::ToCorridoor)
            .chain([Self::ToRoom])
            .collect()
    }
}

#[derive(Clone, Copy)]
struct Action {
    amphipod_id: usize,
    movement: Movement,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State(Vec<Amphipod>);

impl State {
    fn parse(input: &str, big_rooms: bool) -> Self {
        let num_lines = if big_rooms { 4 } else { 2 };
        Self(
            input
                .lines()
                .skip(2)
                .take(num_lines)
                .flat_map(|l| l.chars().skip(3).step_by(2).take(4))
                .map(|c| match c {
                    'A' => AmphipodVariant::A,
                    'B' => AmphipodVariant::B,
                    'C' => AmphipodVariant::C,
                    'D' => AmphipodVariant::D,
                    _ => panic!("{c}"),
                })
                .enumerate()
                .map(|(i, a)| {
                    let room = match i % 4 {
                        0 => AmphipodVariant::A,
                        1 => AmphipodVariant::B,
                        2 => AmphipodVariant::C,
                        3 => AmphipodVariant::D,
                        _ => panic!(),
                    };
                    let position_in_room = i as u32 / 4;
                    Amphipod {
                        variant: a,
                        location: Location::Room(room, position_in_room),
                        can_move: true,
                    }
                })
                .collect::<Vec<_>>(),
        )
    }

    fn is_goal(&self) -> bool {
        self.0.iter().copied().all(|a| a.is_in_correct_room())
    }

    fn is_blocked(&self, start: Location, end: Location) -> bool {
        let (start_x, start_y) = start.to_coords();
        let (end_x, end_y) = end.to_coords();
        if self
            .0
            .iter()
            .map(|a| a.location.to_coords())
            .any(|p| p == (end_x, end_y))
        {
            return true;
        }
        let min_x = start_x.min(end_x);
        let max_x = start_x.max(end_x);

        if self
            .0
            .iter()
            .map(|a| a.location.to_coords())
            .any(|(x, y)| y == 0 && x > min_x && x < max_x)
        {
            return true;
        }

        if self
            .0
            .iter()
            .map(|a| a.location.to_coords())
            .any(|(x, y)| x == start_x && y < start_y)
        {
            return true;
        }

        if self
            .0
            .iter()
            .map(|a| a.location.to_coords())
            .any(|(x, y)| x == end_x && y < end_y)
        {
            return true;
        }

        false
    }

    fn get_free_position(&self, room: AmphipodVariant, big_rooms: bool) -> Option<u32> {
        if self
            .0
            .iter()
            .any(|a| matches!(a.location, Location::Room(r, _) if r == room) && a.variant != room)
        {
            return None;
        }

        let max_pos = if big_rooms { 4 } else { 2 };

        (0..max_pos)
            .filter(|i| {
                self.0
                    .iter()
                    .filter_map(|a| match a.location {
                        Location::Corridoor(_) => None,
                        Location::Room(r, p) => {
                            if r != room {
                                None
                            } else {
                                Some(p)
                            }
                        }
                    })
                    .all(|j| j != *i)
            })
            .next_back()
    }

    fn apply_action(&self, action: Action, big_rooms: bool) -> Option<Self> {
        let to_move = self.0[action.amphipod_id];
        if !to_move.can_move {
            return None;
        }

        match action.movement {
            Movement::ToCorridoor(corridoor_location) => {
                if to_move.location.is_corridoor() {
                    return None;
                }
                let destination = Location::Corridoor(corridoor_location);
                if self.is_blocked(to_move.location, destination) {
                    return None;
                }

                let mut output = self.clone();
                output.0[action.amphipod_id].location = destination;
                Some(output)
            }
            Movement::ToRoom => {
                if to_move.location.is_room() {
                    return None;
                }
                let destination = Location::Room(
                    to_move.variant,
                    self.get_free_position(to_move.variant, big_rooms)?,
                );

                if self.is_blocked(to_move.location, destination) {
                    return None;
                }
                let mut output = self.clone();
                output.0[action.amphipod_id].location = destination;
                output.0[action.amphipod_id].can_move = false;
                Some(output)
            }
        }
    }

    fn get_cost(&self, action: Action, big_rooms: bool) -> u32 {
        let to_move = self.0[action.amphipod_id];
        let start = to_move.location.to_coords();
        let end = match action.movement {
            Movement::ToCorridoor(corridoor_location) => {
                Location::Corridoor(corridoor_location).to_coords()
            }
            Movement::ToRoom => Location::Room(
                to_move.variant,
                self.get_free_position(to_move.variant, big_rooms)
                    .unwrap_or_default(),
            )
            .to_coords(),
        };

        let distance = start.0.abs_diff(end.0) + start.1.abs_diff(end.1);
        distance * to_move.variant.step_cost()
    }

    fn all_actions(&self) -> Vec<Action> {
        (0..self.0.len())
            .cartesian_product(Movement::all())
            .map(|(amphipod_id, movement)| Action {
                amphipod_id,
                movement,
            })
            .collect()
    }

    fn min_cost(self, big_rooms: bool) -> Option<u32> {
        let mut frontier: PriorityQueue<(Self, u32), Reverse<u32>> =
            PriorityQueue::from_iter([((self, (0)), Reverse(0))]);
        let mut visited: HashSet<Self> = HashSet::new();
        loop {
            let ((current_state, current_cost), _) = frontier.pop()?;
            if !visited.insert(current_state.clone()) {
                continue;
            }
            if current_state.is_goal() {
                return Some(current_cost);
            }
            current_state
                .all_actions()
                .into_iter()
                .filter_map(|a| {
                    current_state
                        .apply_action(a, big_rooms)
                        .zip(Some(current_state.get_cost(a, big_rooms)))
                })
                .for_each(|(s, c)| {
                    frontier.push_increase(
                        (s.clone(), current_cost + c),
                        Reverse(current_cost + c + s.heuristic()),
                    );
                })
        }
    }

    fn heuristic(&self) -> u32 {
        self.0
            .iter()
            .filter(|a| !a.is_in_correct_room())
            .map(|a| {
                let pos = a.location.to_coords();
                let dest = Location::Room(a.variant, 0).to_coords();
                let dist = pos.0.abs_diff(dest.0) + pos.1.abs_diff(dest.1);
                dist * a.variant.step_cost()
            })
            .sum()
    }
}

fn solve(mut input: String) -> (u32, u32) {
    let state = State::parse(&input, false);
    let output_1 = state.min_cost(false).unwrap();
    let mut s = input.lines().take(3);
    let mut e = input.lines().skip(3);
    input = format!(
        "{}
  #D#C#B#A#
  #D#B#A#C#
{}",
        s.join("\n"),
        e.join("\n")
    );

    let state = State::parse(&input, true);
    let output_2 = state.min_cost(true).unwrap();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input);
    println!("part 1: {output_1} part 2: {output_2}")
}
