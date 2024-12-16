use crate::position::Position;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Translation {
    pub origin_maps_to: Position,
}

impl From<Position> for Translation {
    fn from(origin_maps_to: Position) -> Self {
        Self { origin_maps_to }
    }
}
