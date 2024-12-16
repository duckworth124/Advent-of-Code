use crate::{
    beacons::Beacons, position::Position, rotation::Rotation, transformation::Transformation,
    translation::Translation,
};
use nom::{bytes, character, multi, sequence, IResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sensor {
    pub beacons: Beacons,
    pub transformation: Option<Vec<(Rotation, Translation)>>,
}

impl Sensor {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        sequence::preceded(
            sequence::tuple((
                bytes::complete::tag("--- scanner "),
                character::complete::i32,
                bytes::complete::tag(" ---\n"),
            )),
            multi::separated_list0(character::complete::newline, Position::parse),
        )
        .map(|positions| {
            let beacons = Beacons::from_iter(positions);
            Self {
                beacons,
                transformation: None,
            }
        })
        .parse(input)
    }

    pub fn get_position(&self) -> Option<Position> {
        let t = self.transformation.as_ref()?;
        Some(t.apply(Position::default()))
    }

    pub fn find_relative_transformation(&self, other: &Self) -> Option<(Rotation, Translation)> {
        self.beacons
            .find_relative_transformation(&other.beacons)
    }
}
