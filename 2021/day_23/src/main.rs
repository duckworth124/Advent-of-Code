use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum AmphipodVariant {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy, Debug)]
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

enum PositionInRoom {
    Up,
    Down,
}

enum Location {
    Room(AmphipodVariant, PositionInRoom),
    Corridoor(CorridoorLocation),
}

struct Amphipod {
    variant: AmphipodVariant,
    location: Location,
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

struct Action {
    amphipod_id: usize,
    movement: Movement,
}

impl Action {
    fn all() -> Vec<Self> {
        (0..8)
            .cartesian_product(Movement::all())
            .map(|(amphipod_id, movement)| Self {
                amphipod_id,
                movement,
            })
            .collect()
    }
}

struct State([Amphipod; 8]);

fn main() {}
