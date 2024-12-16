use std::{cmp::Reverse, collections::HashSet, fs::read_to_string, vec};

use priority_queue::PriorityQueue;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State([AmphipodState; 8]);

impl State {
    fn parse(input: &str) -> Self {
        Self(
            input
                .chars()
                .filter(|c| ['A', 'B', 'C', 'D'].contains(c))
                .enumerate()
                .map(|(i, c)| {
                    let variant = AmphipodVariant::new(c);
                    let room_location = RoomLocation {
                        position_in_room: match i / 4 {
                            0 => PositionInRoom::Up,
                            1 => PositionInRoom::Down,
                            _ => unreachable!(),
                        },

                        room: Room(match i % 4 {
                            0 => AmphipodVariant::A,
                            1 => AmphipodVariant::B,
                            2 => AmphipodVariant::C,
                            3 => AmphipodVariant::D,
                            _ => unreachable!(),
                        }),
                    };
                    let location = Location::InRoom(room_location);

                    AmphipodState { variant, location }
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    fn min_energy(self) -> u32 {
        let mut frontier: PriorityQueue<Node, Reverse<u32>> =
            PriorityQueue::from(vec![(Node::new(self), Reverse(0))]);
        let mut visited: HashSet<Self> = HashSet::new();
        while let Some((current_node, Reverse(current_cost))) = frontier.pop() {
            let current_state = current_node.state;
            if !visited.insert(current_state.clone()) {
                continue;
            }
            if current_state.is_goal() {
                dbg!(current_node.actions);
                return current_cost;
            }

            for action in current_state.get_possible_actions() {
                if let Some((new_state, cost)) = current_state.apply_action(action) {
                    let new_node = Node {
                        state: new_state,
                        actions: { [current_node.actions.clone(), vec![action]].concat() },
                    };

                    frontier.push_increase(new_node, Reverse(cost + current_cost));
                }
            }
        }

        panic!("no path found")
    }

    fn is_goal(&self) -> bool {
        self.0
            .iter()
            .copied()
            .all(AmphipodState::is_on_target)
    }

    fn get_possible_actions(&self) -> Vec<Action> {
        (0..8)
            .flat_map(|i| {
                self.0[i]
                    .get_all_instructions()
                    .into_iter()
                    .map(move |inst| (i, inst))
            })
            .map(|(i, instruction)| Action {
                amphipod_id: i,
                instruction,
            })
            .collect()
    }

    fn apply_action(&self, action: Action) -> Option<(Self, u32)> {
        let amphipod = &self.0[action.amphipod_id];
        let (new_amphipod, cost) = amphipod.apply_instruction(action.instruction, self)?;
        let mut new_state = self.clone();
        new_state.0[action.amphipod_id] = new_amphipod;
        Some((new_state, cost))
    }

    fn is_room_occupied(&self, room: Room) -> bool {
        self.0.iter().any(|a| {
            a.location
                == Location::InRoom(RoomLocation {
                    room,
                    position_in_room: PositionInRoom::Up,
                })
        })
    }

    fn get_bottom_room_occupant(&self, room: Room) -> Option<&AmphipodState> {
        self.0.iter().find(|a| {
            a.location
                == Location::InRoom(RoomLocation {
                    room,
                    position_in_room: PositionInRoom::Down,
                })
        })
    }

    fn is_corridoor_occupied(&self, corridoor_location: CorridoorLocation) -> bool {
        self.0
            .iter()
            .any(|a| a.location == Location::InCorridoor(corridoor_location))
    }

    fn is_path_blocked(&self, from: Location, to: Location) -> bool {
        if let Location::InRoom(room_location) = from {
            if room_location.position_in_room == PositionInRoom::Down
                && self.is_room_occupied(room_location.room)
            {
                return true;
            }
        }
        let x_min = from.to_position().x.min(to.to_position().x);
        let x_max = from.to_position().x.max(to.to_position().x);
        self.0
            .iter()
            .map(|a| a.location.to_position())
            .filter(|p| p.y == 0)
            .map(|p| p.x)
            .any(|x| x_min < x && x < x_max)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Node {
    state: State,
    actions: Vec<Action>,
}

impl Node {
    const fn new(state: State) -> Self {
        Self {
            state,
            actions: vec![],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Action {
    amphipod_id: usize,
    instruction: Instruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    GoToCorridoor(CorridoorLocation),
    GoToRoom(Room),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AmphipodState {
    variant: AmphipodVariant,
    location: Location,
}

impl AmphipodState {
    fn is_on_target(self) -> bool {
        if let Location::InRoom(room_location) = self.location {
            if room_location.room.0 == self.variant {
                return true;
            }
        }

        false
    }

    fn get_all_instructions(self) -> Vec<Instruction> {
        match self.location {
            Location::InRoom(_) => CorridoorLocation::all()
                .into_iter()
                .map(Instruction::GoToCorridoor)
                .collect(),

            Location::InCorridoor(_) => vec![Instruction::GoToRoom(Room(self.variant))],
        }
    }

    fn apply_instruction(self, instruction: Instruction, state: &State) -> Option<(Self, u32)> {
        let target_location = match instruction {
            Instruction::GoToCorridoor(corridoor_location) => corridoor_location.into(),
            Instruction::GoToRoom(room) => RoomLocation {
                room,
                position_in_room: if state.get_bottom_room_occupant(room).is_some() {
                    PositionInRoom::Up
                } else {
                    PositionInRoom::Down
                },
            }
            .into(),
        };

        if state.is_path_blocked(self.location, target_location) {
            return None;
        }

        if self.is_on_target() {
            if let Location::InRoom(room_location) = self.location {
                if room_location.position_in_room == PositionInRoom::Down {
                    return None;
                }
                if state
                    .get_bottom_room_occupant(room_location.room)
                    .is_some_and(|a| a.is_on_target())
                {
                    return None;
                }
            }
        }

        let step_count = self
            .location
            .to_position()
            .manhattan_distance(target_location.to_position());

        match instruction {
            Instruction::GoToRoom(room) => {
                if room.0 != self.variant {
                    return None;
                }

                if state.is_room_occupied(room) {
                    return None;
                }

                let step_count = if let Some(a) = state.get_bottom_room_occupant(room) {
                    if a.variant != self.variant {
                        return None;
                    }
                    step_count
                } else {
                    step_count + 1
                };

                let new = Self {
                    location: target_location,
                    variant: self.variant,
                };

                let cost = self.step_cost() * step_count;

                Some((new, cost))
            }
            Instruction::GoToCorridoor(corridoor_location) => {
                if state.is_corridoor_occupied(corridoor_location) {
                    return None;
                }
                let new = Self {
                    variant: self.variant,
                    location: target_location,
                };
                let cost = self.step_cost() * step_count;

                Some((new, cost))
            }
        }
    }

    const fn step_cost(self) -> u32 {
        match self.variant {
            AmphipodVariant::A => 1,
            AmphipodVariant::B => 10,
            AmphipodVariant::C => 100,
            AmphipodVariant::D => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Location {
    InRoom(RoomLocation),
    InCorridoor(CorridoorLocation),
}

impl Location {
    const fn to_position(self) -> Position {
        match self {
            Self::InRoom(room_location) => room_location.to_position(),
            Self::InCorridoor(corridoor_location) => corridoor_location.to_position(),
        }
    }
}

impl From<CorridoorLocation> for Location {
    fn from(value: CorridoorLocation) -> Self {
        Self::InCorridoor(value)
    }
}

impl From<RoomLocation> for Location {
    fn from(value: RoomLocation) -> Self {
        Self::InRoom(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    const fn manhattan_distance(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RoomLocation {
    room: Room,
    position_in_room: PositionInRoom,
}

impl RoomLocation {
    const fn to_position(self) -> Position {
        let y = match self.position_in_room {
            PositionInRoom::Up => 1,
            PositionInRoom::Down => 2,
        };

        let x = match self.room.0 {
            AmphipodVariant::A => 2,
            AmphipodVariant::B => 4,
            AmphipodVariant::C => 6,
            AmphipodVariant::D => 8,
        };

        Position { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum PositionInRoom {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CorridoorLocation {
    A1,
    A2,
    AB,
    BC,
    CD,
    D1,
    D2,
}

impl CorridoorLocation {
    const fn to_position(self) -> Position {
        let y = 0;
        let x = match self {
            Self::A1 => 0,
            Self::A2 => 1,
            Self::AB => 3,
            Self::BC => 5,
            Self::CD => 7,
            Self::D1 => 9,
            Self::D2 => 10,
        };

        Position { x, y }
    }
}

impl CorridoorLocation {
    fn all() -> Vec<Self> {
        vec![
            Self::A1,
            Self::A2,
            Self::AB,
            Self::BC,
            Self::CD,
            Self::D1,
            Self::D2,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Room(AmphipodVariant);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum AmphipodVariant {
    A,
    B,
    C,
    D,
}

impl AmphipodVariant {
    fn new(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            _ => unreachable!(),
        }
    }
}

fn solve(path: &str) -> u32 {
    let input = read_to_string(path).unwrap();
    let state = State::parse(&input);

    state.min_energy()
}

fn main() {
    let output_1 = solve("input");
    println!("part 1: {output_1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harder() {
        let input = "ABDC ABCD";
        let state = State::parse(input);
        assert_eq!(state.min_energy(), 200 + 4000 + 400);
    }

    #[test]
    fn positioning() {
        let l = Location::InRoom(RoomLocation {
            position_in_room: PositionInRoom::Up,
            room: Room(AmphipodVariant::A),
        });

        assert_eq!(l.to_position(), Position { y: 1, x: 2 });

        let l = Location::InCorridoor(CorridoorLocation::BC);
        assert_eq!(l.to_position(), Position { x: 5, y: 0 });
    }

    #[test]
    fn very_difficult() {
        let input = "DBCD ABCA";
        let state = State::parse(input);
        state.min_energy();
    }
}
