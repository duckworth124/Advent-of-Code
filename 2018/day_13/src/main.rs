use std::{collections::HashSet, fs::read_to_string};

use itertools::iproduct;

#[derive(Clone, Copy)]
enum Rail {
    Straight,
    UpLeft,
    UpRight,
    Intersection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn to_coords(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    const fn rotate_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    const fn rotate_right(self) -> Self {
        self.rotate_left().rotate_left().rotate_left()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IntersectionAction {
    TurnLeft,
    GoStraight,
    TurnRight,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cart {
    position: Position,
    facing: Direction,
    intersection_action: IntersectionAction,
}

impl Cart {
    const fn step(&mut self) {
        let (dx, dy) = self.facing.to_coords();
        self.position.x += dx;
        self.position.y += dy
    }

    const fn next_intersection_action(&mut self) -> IntersectionAction {
        let output = self.intersection_action;
        self.intersection_action = match output {
            IntersectionAction::TurnLeft => IntersectionAction::GoStraight,
            IntersectionAction::GoStraight => IntersectionAction::TurnRight,
            IntersectionAction::TurnRight => IntersectionAction::TurnLeft,
        };

        output
    }
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<Option<Rail>>>,
    carts: Vec<Cart>,
    occupied: HashSet<Position>,
}

impl Grid {
    fn step(&mut self, remove_crashes: bool) -> Option<Position> {
        self.carts.sort_by_key(|c| (c.position.y, c.position.x));
        let mut to_remove = vec![];
        for cart_id in 0..self.carts.len() {
            if to_remove.contains(&cart_id) {
                continue;
            }
            let cart = &mut self.carts[cart_id];
            self.occupied.remove(&cart.position);
            cart.step();
            if !self.occupied.insert(cart.position) {
                self.occupied.remove(&cart.position);
                if remove_crashes {
                    let cart = *cart;
                    to_remove.push(cart_id);
                    to_remove.push(
                        self.carts
                            .iter()
                            .enumerate()
                            .position(|(i, c)| i != cart_id && c.position == cart.position)
                            .unwrap(),
                    );
                } else {
                    return Some(cart.position);
                }
            }
            let cart = &mut self.carts[cart_id];
            let position = cart.position;
            let facing = cart.facing;
            let rail = self.get(position).unwrap();
            let cart = &mut self.carts[cart_id];
            let new_facing = match rail {
                Rail::UpLeft => match facing {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                },
                Rail::UpRight => match facing {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                },
                Rail::Intersection => {
                    let action = cart.next_intersection_action();
                    match action {
                        IntersectionAction::TurnLeft => facing.rotate_left(),
                        IntersectionAction::GoStraight => facing,
                        IntersectionAction::TurnRight => facing.rotate_right(),
                    }
                }
                Rail::Straight => facing,
            };

            cart.facing = new_facing;
        }

        to_remove.sort();
        to_remove.reverse();
        for c in to_remove {
            self.carts.remove(c);
        }

        if remove_crashes && self.carts.len() == 1 {
            return Some(self.carts[0].position);
        }

        None
    }

    fn get(&self, position: Position) -> Option<Rail> {
        let y: usize = position.y.try_into().ok()?;
        let x: usize = position.x.try_into().ok()?;
        *self.grid.get(y)?.get(x)?
    }

    fn parse(input: &str) -> Self {
        let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let grid: Vec<Vec<Option<Rail>>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '/' => Some(Rail::UpLeft),
                        '\\' => Some(Rail::UpRight),
                        '+' => Some(Rail::Intersection),
                        ' ' => None,
                        _ => Some(Rail::Straight),
                    })
                    .collect()
            })
            .collect();

        let carts: Vec<Cart> = iproduct!(0..grid[0].len(), 0..grid.len())
            .map(|(x, y)| (x, y, chars[y][x]))
            .filter_map(|(x, y, c)| {
                let facing = match c {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => return None,
                };
                let position = Position {
                    x: x as i32,
                    y: y as i32,
                };

                Some(Cart {
                    position,
                    facing,
                    intersection_action: IntersectionAction::TurnLeft,
                })
            })
            .collect();

        let occupied = carts.iter().copied().map(|c| c.position).collect();

        Self {
            grid,
            carts,
            occupied,
        }
    }
}

fn solve(input: &str) -> ((i32, i32), (i32, i32)) {
    let mut grid = Grid::parse(input);
    let mut grid_2 = grid.clone();
    let position = loop {
        if let Some(x) = grid.step(false) {
            break x;
        }

        //dbg!(&grid.carts);
    };

    let output_1 = (position.x, position.y);

    let position = loop {
        if let Some(x) = grid_2.step(true) {
            break x;
        }
    };

    let output_2 = (position.x, position.y);
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1:?} part 2: {output_2:?}")
}
