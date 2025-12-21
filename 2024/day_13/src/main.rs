use num::Rational64;
use std::{fs::read_to_string, time::Instant};

#[derive(Clone, Copy)]
struct Machine {
    a: [i64; 2],
    b: [i64; 2],
    target: [i64; 2],
}

impl Machine {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let (l, r) = lines.next().unwrap().split_once(',').unwrap();
        let a = [
            l.strip_prefix("Button A: X+").unwrap().parse().unwrap(),
            r.strip_prefix(" Y+").unwrap().parse().unwrap(),
        ];

        let (l, r) = lines.next().unwrap().split_once(',').unwrap();
        let b = [
            l.strip_prefix("Button B: X+").unwrap().parse().unwrap(),
            r.strip_prefix(" Y+").unwrap().parse().unwrap(),
        ];

        let (l, r) = lines.next().unwrap().split_once(',').unwrap();
        let target = [
            l.strip_prefix("Prize: X=").unwrap().parse().unwrap(),
            r.strip_prefix(" Y=").unwrap().parse().unwrap(),
        ];

        Self { a, b, target }
    }

    fn get_tickets(self) -> Option<i64> {
        let equation_system = EquationSystem {
            matrix: [
                [self.a[0].into(), self.b[0].into()],
                [self.a[1].into(), self.b[1].into()],
            ],
            rhs: self.target.map(|x| x.into()),
        };
        let output = equation_system.solve();
        if output.iter().any(|&x| !x.is_integer() || x < 0.into()) {
            return None;
        }
        Some((output[0] * 3 + output[1]).to_integer())
    }
}

struct EquationSystem {
    matrix: [[Rational64; 2]; 2],
    rhs: [Rational64; 2],
}

impl EquationSystem {
    fn scale_top(&mut self, scale_factor: Rational64) {
        self.matrix[0].iter_mut().for_each(|x| *x *= scale_factor);
        self.rhs[0] *= scale_factor;
    }

    fn scale_bottom(&mut self, scale_factor: Rational64) {
        self.matrix[1].iter_mut().for_each(|x| *x *= scale_factor);
        self.rhs[1] *= scale_factor;
    }

    fn sub_top_from_bottom(&mut self) {
        self.matrix[1][0] -= self.matrix[0][0];
        self.matrix[1][1] -= self.matrix[0][1];
        self.rhs[1] -= self.rhs[0]
    }

    fn sub_bottom_from_top(&mut self) {
        self.matrix[0][0] -= self.matrix[1][0];
        self.matrix[0][1] -= self.matrix[1][1];
        self.rhs[0] -= self.rhs[1]
    }

    fn solve(mut self) -> [Rational64; 2] {
        self.scale_top(self.matrix[1][0] / self.matrix[0][0]);
        self.sub_top_from_bottom();
        self.scale_top(self.matrix[0][0].recip());

        self.scale_bottom(self.matrix[0][1] / self.matrix[1][1]);
        self.sub_bottom_from_top();
        self.scale_bottom(self.matrix[1][1].recip());

        self.rhs
    }
}

fn solve(input: &str) -> (i64, i64) {
    let mut output_1 = 0;
    let mut output_2 = 0;
    for mut m in input.split("\n\n").map(Machine::parse) {
        output_1 += m.get_tickets().unwrap_or_default();

        m.target.iter_mut().for_each(|x| *x += 10000000000000);
        output_2 += m.get_tickets().unwrap_or_default();
    }
    (output_1, output_2)
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
