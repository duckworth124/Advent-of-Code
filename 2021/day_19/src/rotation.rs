use itertools::iproduct;

use crate::direction::Direction;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rotation {
    pub positive_x_maps_to: Direction,
    pub positive_y_maps_to: Direction,
}

impl Rotation {
    pub fn all() -> Vec<Self> {
        iproduct!(Direction::all(), Direction::all())
            .map(|(positive_x_maps_to, positive_y_maps_to)| Self {
                positive_y_maps_to,
                positive_x_maps_to,
            })
            .filter(|r| {
                r.positive_x_maps_to
                    .is_orthogonal(r.positive_y_maps_to)
            })
            .collect()
    }

    pub fn positive_z_maps_to(self) -> Direction {
        self.positive_x_maps_to * self.positive_y_maps_to
    }
}

#[cfg(test)]
mod tests {
    use crate::{axis::Axis, axis_direction::AxisDirection};

    use super::*;

    #[test]
    fn it_works() {
        let d = Rotation {
            positive_x_maps_to: Direction {
                axis: Axis::X,
                direction_along_axis: AxisDirection::Positive,
            },
            positive_y_maps_to: Direction {
                axis: Axis::Y,
                direction_along_axis: AxisDirection::Negative,
            },
        };

        assert_eq!(
            d.positive_z_maps_to(),
            Direction {
                axis: Axis::Z,
                direction_along_axis: AxisDirection::Negative
            }
        )
    }
}
