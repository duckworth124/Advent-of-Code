use crate::{position::Position, rotation::Rotation, translation::Translation};

pub trait Transformation {
    fn apply(&self, position: Position) -> Position;
}

impl Transformation for Translation {
    fn apply(&self, position: Position) -> Position {
        position + self.origin_maps_to
    }
}

impl Transformation for Rotation {
    fn apply(&self, position: Position) -> Position {
        let x_image_base = self.positive_x_maps_to.base_position() * position.x;
        let y_image_base = self.positive_y_maps_to.base_position() * position.y;
        let positive_z_maps_to = self.positive_z_maps_to();
        let z_image_base = positive_z_maps_to.base_position() * position.z;

        x_image_base + y_image_base + z_image_base
    }
}

impl<T: Transformation> Transformation for Vec<T> {
    fn apply(&self, position: Position) -> Position {
        self.iter()
            .fold(position, |current, transformation| {
                transformation.apply(current)
            })
    }
}

impl Transformation for (Rotation, Translation) {
    fn apply(&self, position: Position) -> Position {
        self.1.apply(self.0.apply(position))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{axis::Axis, axis_direction::AxisDirection, direction::Direction};

    #[test]
    fn it_works() {
        let p = Position::from((1, 1, 1));
        let r = Rotation {
            positive_x_maps_to: Direction {
                axis: Axis::X,
                direction_along_axis: AxisDirection::Positive,
            },
            positive_y_maps_to: Direction {
                axis: Axis::Y,
                direction_along_axis: AxisDirection::Positive,
            },
        };

        let output = r.apply(p);
        assert_eq!(output, Position::from((1, 1, 1)));

        let p = Position::from((1, 1, 1));
        let r = Rotation {
            positive_x_maps_to: Direction {
                axis: Axis::X,
                direction_along_axis: AxisDirection::Negative,
            },
            positive_y_maps_to: Direction {
                axis: Axis::Y,
                direction_along_axis: AxisDirection::Positive,
            },
        };

        let output = r.apply(p);
        assert_eq!(output, Position::from((-1, 1, -1)))
    }
}
