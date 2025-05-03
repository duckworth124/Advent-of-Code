use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn step(self, (dx, dy): (i32, i32)) -> Option<Self> {
        let (x, y) = (self.x + dx, self.y + dy);
        if (x - 2).abs() + (y - 2).abs() > 2 {
            return None;
        }

        Some(Self { x, y })
    }

    fn to_char(self) -> char {
        match (self.x, self.y) {
            (2, 0) => '1',
            (1, 1) => '2',
            (2, 1) => '3',
            (3, 1) => '4',
            (0, 2) => '5',
            (1, 2) => '6',
            (2, 2) => '7',
            (3, 2) => '8',
            (4, 2) => '9',
            (1, 3) => 'A',
            (2, 3) => 'B',
            (3, 3) => 'C',
            (2, 4) => 'D',
            _ => panic!("invalid position: {} {}", self.x, self.y),
        }
    }
}

fn solve(input: &str) -> (i32, String) {
    let directions: Vec<Vec<(i32, i32)>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'U' => (0, -1),
                    'D' => (0, 1),
                    'L' => (-1, 0),
                    'R' => (1, 0),
                    _ => panic!("unrecognized character: {c}"),
                })
                .collect()
        })
        .collect();

    let output_1 = directions
        .iter()
        .scan((1, 1), |(x, y), ds| {
            for (dx, dy) in ds {
                *x = (*x + dx).clamp(0, 2);
                *y = (*y + dy).clamp(0, 2);
            }
            Some((*x, *y))
        })
        .map(|(x, y)| 3 * y + x + 1)
        .fold(0, |acc, d| acc * 10 + d);

    let output_2 = directions
        .iter()
        .scan(Position { x: 0, y: 2 }, |p, ds| {
            for &d in ds {
                *p = p.step(d).unwrap_or(*p);
            }

            Some(*p)
        })
        .map(|p| p.to_char())
        .collect();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
