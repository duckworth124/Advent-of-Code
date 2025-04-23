use std::fs::read_to_string;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_coords(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("unrecognized character: {c}"),
                })
                .map(|d| d.to_coords())
        })
        .scan((1, 1), |(x, y), ds| {
            for (dx, dy) in ds {
                *x = (*x + dx).clamp(0, 2);
                *y = (*y + dy).clamp(0, 2);
            }
            Some((*x, *y))
        })
        .map(|(x, y)| 3 * y + x + 1)
        .inspect(|x| println!("{x}"))
        .fold(0, |acc, d| acc * 10 + d)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = solve(&input);
    println!("part 1: {output_1}")
}
