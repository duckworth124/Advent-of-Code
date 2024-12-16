use std::ops::Mul;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AxisDirection {
    Positive,
    Negative,
}

impl AxisDirection {
    pub fn all() -> Vec<Self> {
        vec![Self::Positive, Self::Negative]
    }
}

impl Mul for AxisDirection {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

impl From<AxisDirection> for i32 {
    fn from(val: AxisDirection) -> Self {
        match val {
            AxisDirection::Positive => 1,
            AxisDirection::Negative => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn it_works() {
        let mut x = AxisDirection::all();
        assert_eq!(x, vec![AxisDirection::Positive, AxisDirection::Negative]);
        x = x
            .into_iter()
            .map(|a| a * AxisDirection::Negative)
            .collect();
        assert_eq!(x, vec![AxisDirection::Negative, AxisDirection::Positive]);
        let y = x.into_iter().map(i32::from).collect_vec();
        assert_eq!(y, vec![-1, 1])
    }
}
