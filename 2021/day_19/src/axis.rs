use crate::position::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn all() -> Vec<Self> {
        vec![Self::X, Self::Y, Self::Z]
    }

    pub fn base_position(self) -> Position {
        let coords = match self {
            Axis::X => (1, 0, 0),
            Axis::Y => (0, 1, 0),
            Axis::Z => (0, 0, 1),
        };

        coords.into()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn it_works() {
        let positions = Axis::all()
            .into_iter()
            .map(Axis::base_position)
            .collect_vec();
        assert_eq!(positions[0], Position { x: 1, y: 0, z: 0 });
        assert_eq!(positions[1], Position { x: 0, y: 1, z: 0 });
        assert_eq!(positions[2], Position { x: 0, y: 0, z: 1 });
    }
}
