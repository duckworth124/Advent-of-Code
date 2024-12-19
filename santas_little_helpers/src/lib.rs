#[allow(dead_code)]

mod lib {
    pub mod positions_and_friends {
        use std::ops::{Add, Neg, Sub};

        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
        struct Position {
            x: i32,
            y: i32,
        }

        impl Position {
            fn step(self, direction: Direction) -> Self {
                self + direction.into()
            }
        }

        impl From<Direction> for Position {
            fn from(value: Direction) -> Self {
                let (x, y) = match value {
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                };

                Self { x, y }
            }
        }

        impl Add for Position {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                let (x, y) = (self.x + rhs.x, self.y + rhs.y);
                Self { x, y }
            }
        }

        impl Neg for Position {
            type Output = Self;
            fn neg(self) -> Self::Output {
                let (x, y) = (-self.x, -self.y);
                Self { x, y }
            }
        }

        impl Sub for Position {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                self + -rhs
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum Direction {
            Up,
            Down,
            Left,
            Right,
        }

        impl Direction {
            fn rotate_right(self) -> Self {
                match self {
                    Self::Up => Self::Right,
                    Self::Right => Self::Down,
                    Self::Down => Self::Left,
                    Self::Left => Self::Up,
                }
            }

            fn rotate_left(self) -> Self {
                self.rotate_right().rotate_right().rotate_right()
            }

            fn opposite(self) -> Self {
                self.rotate_right().rotate_right()
            }
        }
    }

    pub mod parsing {
        use nom::{
            branch::alt,
            character::complete::{anychar, i32, u32},
            multi::many0,
            Parser,
        };

        fn all_numbers_i32(input: &str) -> Vec<i32> {
            many0(alt((i32::<&str, ()>.map(Some), anychar.map(|_| None))))
                .map(|x| x.into_iter().flatten().collect())
                .parse(input)
                .unwrap()
                .1
        }

        fn all_numbers_u32(input: &str) -> Vec<u32> {
            many0(alt((u32::<&str, ()>.map(Some), anychar.map(|_| None))))
                .map(|x| x.into_iter().flatten().collect())
                .parse(input)
                .unwrap()
                .1
        }
    }
}
