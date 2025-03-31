use std::fs::read_to_string;

struct Icpu {
    data: Vec<usize>,
    pc: usize,
}

impl Icpu {
    fn new(data: Vec<usize>) -> Self {
        Self { data, pc: 0 }
    }

    fn step(&mut self) -> bool {
        let op = self.data[self.pc];
        let val1 = self.data[self.pc + 1];
        let val2 = self.data[self.pc + 2];
        let out = self.data[self.pc + 3];
        self.pc += 4;

        match op {
            1 => self.data[out] = self.data[val1] + self.data[val2],

            2 => self.data[out] = self.data[val1] * self.data[val2],

            99 => return false,
            _ => {
                panic!("unrecognized opcode : {op}")
            }
        };

        true
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

const TARGET: usize = 19690720;

fn solve(data: Vec<usize>) -> (usize, usize) {
    let mut icpu = Icpu::new(data.clone());
    icpu.data[1] = 12;
    icpu.data[2] = 2;
    icpu.run();
    let output_1 = icpu.data[0];

    let mut output_2 = None;
    'outer: for i in 0..=99 {
        for j in 0..=99 {
            icpu = Icpu::new(data.clone());
            icpu.data[1] = i;
            icpu.data[2] = j;
            icpu.run();
            if icpu.data[0] == TARGET {
                output_2 = Some(100 * i + j);
                break 'outer;
            }
        }
    }

    (output_1, output_2.unwrap())
}

fn main() {
    let input = read_to_string("input").unwrap();
    let data: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let (output_1, output_2) = solve(data);

    println!("part 1: {output_1} part 2: {output_2}")
}
