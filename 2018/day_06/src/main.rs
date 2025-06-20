use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn distance(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn neighbours(self) -> [Self; 4] {
        [
            (self.x - 1, self.y),
            (self.x + 1, self.y),
            (self.x, self.y - 1),
            (self.x, self.y + 1),
        ]
        .map(|(x, y)| Self { x, y })
    }
}

fn get_area(position: Position, positions: &[Position]) -> Option<usize> {
    let mut output = 0;
    let mut frontier: VecDeque<Position> = VecDeque::from_iter([position]);
    let mut visited = HashSet::new();
    let max_x = positions.iter().map(|p| p.x).max().unwrap();
    let min_x = positions.iter().map(|p| p.x).min().unwrap();
    let max_y = positions.iter().map(|p| p.y).max().unwrap();
    let min_y = positions.iter().map(|p| p.y).min().unwrap();

    while let Some(current_position) = frontier.pop_front() {
        if !visited.insert(current_position) {
            continue;
        }
        let distance = current_position.distance(position);
        if positions
            .iter()
            .copied()
            .filter(|p| *p != position)
            .map(|p| p.distance(current_position))
            .any(|d| d <= distance)
        {
            continue;
        }

        if current_position.x >= max_x {
            return None;
        }
        if current_position.y >= max_y {
            return None;
        }
        if current_position.x <= min_x {
            return None;
        }
        if current_position.y <= min_y {
            return None;
        }

        output += 1;
        frontier.extend(current_position.neighbours());
    }

    Some(output)
}

fn find_start(positions: &[Position]) -> Position {
    let mut current_position = Position { x: 0, y: 0 };
    loop {
        let total_distance: u32 = positions
            .iter()
            .copied()
            .map(|p| p.distance(current_position))
            .sum();
        if total_distance < 10_000 {
            break current_position;
        }
        current_position = current_position
            .neighbours()
            .into_iter()
            .find(|&p| positions.iter().map(|p2| p2.distance(p)).sum::<u32>() < total_distance)
            .unwrap();
    }
}

fn get_area_part_2(start: Position, positions: &[Position]) -> usize {
    let mut frontier: VecDeque<Position> = VecDeque::from([start]);
    let mut visited: HashSet<Position> = HashSet::new();
    let mut output = 0;
    while let Some(current_position) = frontier.pop_front() {
        if !visited.insert(current_position) {
            continue;
        }
        let total_distance: u32 = positions
            .iter()
            .copied()
            .map(|p| p.distance(current_position))
            .sum();
        if total_distance >= 10_000 {
            continue;
        }
        output += 1;
        frontier.extend(current_position.neighbours());
    }
    output
}

fn solve(input: &str) -> (usize, usize) {
    let positions: Vec<Position> = input
        .lines()
        .map(|line| line.split_once(", ").unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .map(|(x, y)| Position { x, y })
        .collect();

    let output_1 = positions
        .iter()
        .copied()
        .map(|p| (p, &positions))
        .filter_map(|(p, positions)| get_area(p, positions))
        .max()
        .unwrap();

    let start = find_start(&positions);
    let output_2 = get_area_part_2(start, &positions);

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
