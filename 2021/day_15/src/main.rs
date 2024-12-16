use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> [Self; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: Self) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }

    fn step(&self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Self {
                y: self.y.checked_sub(1)?,
                ..*self
            },

            Direction::Down => Self {
                y: self.y + 1,
                ..*self
            },

            Direction::Left => Self {
                x: self.x.checked_sub(1)?,
                ..*self
            },

            Direction::Right => Self {
                x: self.x + 1,
                ..*self
            },
        })
    }
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    position: Position,
    cost: u32,
}

impl State {
    fn new(position: Position, cost: u32) -> Self {
        Self { position, cost }
    }

    fn is_goal(&self, grid: &Grid) -> bool {
        let end_position = grid.end_position();
        self.position == end_position
    }

    fn expected_future_cost(&self, grid: &Grid) -> u32 {
        let end_position = grid.end_position();
        end_position.manhattan_distance(self.position)
    }

    fn apply_action(&self, direction: Direction, grid: &Grid) -> Option<Self> {
        let position = self.position.step(direction)?;
        let cost = self.cost + grid.get_cost(position)?;
        Some(Self::new(position, cost))
    }
}

struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        Grid(grid)
    }

    fn end_position(&self) -> Position {
        let width = self.0[0].len();
        let height = self.0.len();
        Position::new(width - 1, height - 1)
    }

    fn get_cost(&self, position: Position) -> Option<u32> {
        self.0.get(position.y)?.get(position.x).copied()
    }

    fn expand(&self) -> Self {
        let grid: Vec<Vec<u32>> = self
            .0
            .iter()
            .map(|v| {
                [v; 5]
                    .iter()
                    .enumerate()
                    .flat_map(|(i, v)| {
                        v.iter().map(move |x| {
                            let mut output = x + i as u32;
                            while output >= 10 {
                                output -= 9
                            }
                            output
                        })
                    })
                    .collect()
            })
            .collect();

        let grid = vec![grid; 5]
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                v.iter().map(move |v| {
                    v.iter()
                        .map(move |x| {
                            let mut output = x + i as u32;
                            while output >= 10 {
                                output -= 9
                            }
                            output
                        })
                        .collect()
                })
            })
            .collect();
        Grid(grid)
    }

    fn min_risk(&self) -> u32 {
        let mut frontier: PriorityQueue<State, Reverse<u32>> =
            PriorityQueue::from(vec![(State::new(Position::new(0, 0), 0), Reverse(0))]);
        let mut visited: HashSet<Position> = HashSet::new();

        while let Some((current_state, _)) = frontier.pop() {
            if !visited.insert(current_state.position) {
                continue;
            }

            if current_state.is_goal(self) {
                return current_state.cost;
            }

            for new_state in Direction::all()
                .iter()
                .flat_map(|&d| current_state.apply_action(d, self))
            {
                let priority = new_state.cost + new_state.expected_future_cost(self);
                frontier.push_increase(new_state, Reverse(priority));
            }
        }
        panic!("no path found")
    }
}

fn solve(path: &str) -> (u32, u32) {
    let input = read_to_string(path).unwrap();
    let grid = Grid::new(&input);
    let output_1 = grid.min_risk();

    let big_grid = grid.expand();
    let output_2 = big_grid.min_risk();
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}

#[test]
fn practice_input() {
    let (output_1, output_2) = solve("practice");
    assert_eq!(output_1, 40);
    assert_eq!(output_2, 315);
}
