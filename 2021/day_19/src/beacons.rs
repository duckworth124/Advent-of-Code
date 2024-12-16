use crate::{
    position::Position, rotation::Rotation, transformation::Transformation,
    translation::Translation,
};
use itertools::Itertools;
use std::{collections::HashSet, ops::BitAnd};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Beacons(HashSet<Position>);

impl Beacons {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> impl Iterator<Item = &Position> {
        self.0.iter()
    }

    pub fn transform(self, transformation: impl Transformation) -> Self {
        self.into_iter()
            .map(|p| transformation.apply(p))
            .collect()
    }

    fn get_all_translations(&self, other: &Self) -> Vec<Translation> {
        self.iter()
            .cartesian_product(other)
            .map(|(&here, &there)| there - here)
            .map(Into::into)
            .collect()
    }

    fn find_relative_translation(&self, other: &Self) -> Option<Translation> {
        self.get_all_translations(other)
            .into_iter()
            .find(|&translation| ((&self.clone().transform(translation) & other).len()) >= 12)
    }

    pub fn find_relative_transformation(&self, other: &Self) -> Option<(Rotation, Translation)> {
        Rotation::all().into_iter().find_map(|rotation| {
            let translation = self
                .clone()
                .transform(rotation)
                .find_relative_translation(other)?;
            Some((rotation, translation))
        })
    }
}

impl From<HashSet<Position>> for Beacons {
    fn from(value: HashSet<Position>) -> Self {
        Self(value)
    }
}

impl<T: Into<Position>> FromIterator<T> for Beacons {
    fn from_iter<It: IntoIterator<Item = T>>(iter: It) -> Self {
        iter.into_iter()
            .map(Into::into)
            .collect::<HashSet<Position>>()
            .into()
    }
}

impl IntoIterator for Beacons {
    type Item = Position;

    type IntoIter = <HashSet<Position> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Beacons {
    type IntoIter = <&'a HashSet<Position> as IntoIterator>::IntoIter;
    type Item = &'a Position;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl BitAnd for &Beacons {
    type Output = Beacons;

    fn bitand(self, rhs: Self) -> Self::Output {
        (&self.0 & &rhs.0).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        axis::Axis, axis_direction::AxisDirection, direction::Direction, sensors::Sensors,
    };
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn it_works() {
        let input = read_to_string("practice").unwrap();
        let sensors = Sensors::parse(&input).unwrap().1;
        let sensor_0 = sensors.0[0].clone();
        let beacons_0 = sensor_0.beacons;
        let sensor_1 = sensors.0[1].clone();
        let beacons_1 = sensor_1.beacons;

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

        let beacons_0_rotated = beacons_0.transform(r);
        dbg!(&beacons_0_rotated);
        let t = beacons_0_rotated.find_relative_translation(&beacons_1);
        assert!(t.is_some())
    }
}
