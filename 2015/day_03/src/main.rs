use std::{collections::HashSet, fs::read_to_string, ops::AddAssign, time::Instant};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position([i32; 2]);

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0.iter_mut().zip(rhs.0).for_each(|(x, y)| *x += y);
    }
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let mut santa_1 = Position::default();
    let mut santa_2 = Position::default();
    let mut robo_santa = Position::default();
    let mut should_santa_move = false;
    let mut visited_1: HashSet<Position> = HashSet::from([Position::default()]);
    let mut visited_2: HashSet<Position> = HashSet::from([Position::default()]);

    for d in input.chars().map(|c| match c {
        '^' => Position([0, -1]),
        'v' => Position([0, 1]),
        '<' => Position([-1, 0]),
        '>' => Position([1, 0]),
        _ => panic!("unrecognized char: {c:?}"),
    }) {
        santa_1 += d;
        visited_1.insert(santa_1);

        if should_santa_move {
            santa_2 += d;
            visited_2.insert(santa_2);
        } else {
            robo_santa += d;
            visited_2.insert(robo_santa);
        }
        should_santa_move = !should_santa_move;
    }
    (visited_1.len(), visited_2.len())
}

fn main() {
    let time = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
