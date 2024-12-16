use std::ops::Mul;

use itertools::iproduct;

use crate::{axis::Axis, axis_direction::AxisDirection, position::Position, rotation::Rotation};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Direction {
    pub axis: Axis,
    pub direction_along_axis: AxisDirection,
}

impl Direction {
    pub fn all() -> Vec<Self> {
        iproduct!(Axis::all(), AxisDirection::all())
            .map(|(axis, direction_along_axis)| Self {
                axis,
                direction_along_axis,
            })
            .collect()
    }

    pub fn base_position(self) -> Position {
        self.axis.base_position() * self.direction_along_axis.into()
    }

    pub fn rotate(self, rotation: Rotation) -> Self {
        let image_direction = match self.axis {
            Axis::X => rotation.positive_x_maps_to,
            Axis::Y => rotation.positive_y_maps_to,
            Axis::Z => rotation.positive_z_maps_to(),
        };

        let axis = image_direction.axis;
        let direction_along_axis = image_direction.direction_along_axis * self.direction_along_axis;

        Self {
            axis,
            direction_along_axis,
        }
    }

    pub fn is_orthogonal(self, other: Self) -> bool {
        self.axis != other.axis
    }
}

impl Mul for Direction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.is_orthogonal(rhs), "product of parallel!");
        let axis = Axis::all()
            .into_iter()
            .find(|&a| a != self.axis && a != rhs.axis)
            .unwrap();

        let direction_along_axis = match (self.axis, rhs.axis) {
            (Axis::X, Axis::Y) | (Axis::Y, Axis::Z) | (Axis::Z, Axis::X) => AxisDirection::Positive,

            _ => AxisDirection::Negative,
        } * self.direction_along_axis
            * rhs.direction_along_axis;

        Self {
            axis,
            direction_along_axis,
        }
    }
}
