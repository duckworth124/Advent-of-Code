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

            fn is_in_bounds(self, width: i32, height: i32) -> bool {
                (0..width).contains(&self.x) && (0..height).contains(&self.y)
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
            const fn rotate_right(self) -> Self {
                match self {
                    Self::Up => Self::Right,
                    Self::Right => Self::Down,
                    Self::Down => Self::Left,
                    Self::Left => Self::Up,
                }
            }

            const fn rotate_left(self) -> Self {
                self.rotate_right().rotate_right().rotate_right()
            }

            const fn opposite(self) -> Self {
                self.rotate_right().rotate_right()
            }
        }
    }

    pub mod parsing {
        use nom::{
            character::complete::{anychar, i32, u32},
            multi::{many0, many_till},
            Parser,
        };

        fn all_numbers_i32(input: &str) -> Vec<i32> {
            many0(many_till(anychar::<_, ()>, i32).map(|(_, x)| x))
                .parse(input)
                .unwrap()
                .1
        }

        fn all_numbers_u32(input: &str) -> Vec<u32> {
            many0(many_till(anychar::<_, ()>, u32).map(|(_, x)| x))
                .parse(input)
                .unwrap()
                .1
        }
    }
}
