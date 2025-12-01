use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

#[derive(Default, Clone)]
struct Icpu {
    data: HashMap<i128, i128>,
    pc: i128,
    inputs: VecDeque<i128>,
    outputs: Vec<i128>,
    relative_base: i128,
}

impl Icpu {
    fn new(data: Vec<i128>) -> Self {
        let data = data
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i as i128, x))
            .collect();
        Self {
            data,
            ..Default::default()
        }
    }

    fn step(&mut self) -> bool {
        let op = self.get(self.pc);
        match op % 100 {
            1 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                self.write(3, op, par1 + par2);
                self.pc += 4;
            }

            2 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                self.write(3, op, par1 * par2);
                self.pc += 4;
            }

            3 => {
                let input = self.inputs.pop_front().unwrap();
                self.write(1, op, input);
                self.pc += 2;
            }

            4 => {
                let par = self.get_param(1, op);
                self.outputs.push(par);
                self.pc += 2;
            }

            5 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                if par1 != 0 {
                    self.pc = par2;
                } else {
                    self.pc += 3
                }
            }

            6 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                if par1 == 0 {
                    self.pc = par2;
                } else {
                    self.pc += 3
                }
            }

            7 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);

                let output = (par1 < par2) as i128;
                self.write(3, op, output);
                self.pc += 4;
            }

            8 => {
                let par1 = self.get_param(1, op);
                let par2 = self.get_param(2, op);
                let output = (par1 == par2) as i128;
                self.write(3, op, output);
                self.pc += 4;
            }

            9 => {
                let par = self.get_param(1, op);
                self.relative_base += par;
                self.pc += 2;
            }

            99 => return false,

            _ => {
                panic!("unrecoginized opcode: {op}")
            }
        };

        true
    }

    fn get_param(&self, param_number: i128, op: i128) -> i128 {
        let mode = (op / (10_i128.pow(1 + param_number as u32))) % 10;
        let val = self.data[&(self.pc + param_number)];
        if mode == 1 {
            val
        } else if mode == 2 {
            let address = val + self.relative_base;
            self.get(address)
        } else {
            let address = val;
            self.get(address)
        }
    }

    fn write(&mut self, param_number: i128, op: i128, value: i128) {
        let mode = (op / (10_i128.pow(1 + param_number as u32))) % 10;
        let val = self.data[&(self.pc + param_number)];
        if mode == 2 {
            let address = val + self.relative_base;
            *self.get_mut(address) = value
        } else {
            let address = val;
            *self.get_mut(address) = value
        }
    }

    fn get_mut(&mut self, address: i128) -> &mut i128 {
        self.data.entry(address).or_default()
    }

    fn get(&self, address: i128) -> i128 {
        self.data.get(&address).copied().unwrap_or_default()
    }

    fn get_output(&mut self) -> Option<i128> {
        loop {
            let output = self.outputs.pop();
            if output.is_some() {
                return output;
            }
            if !self.step() {
                return None;
            }
        }
    }
}

fn solve(input: &str) -> (usize, u32) {
    let data: Vec<i128> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut frontier = VecDeque::from([vec![1], vec![2], vec![3], vec![4]]);
    let mut seen = HashSet::new();
    let mut map = HashMap::new();
    let mut distance = None;
    let mut origin = None;
    while let Some(path) = frontier.pop_front() {
        let position = path
            .iter()
            .map(|x| match x {
                1 => (0, -1),
                2 => (0, 1),
                3 => (-1, 0),
                4 => (1, 0),
                _ => panic!(),
            })
            .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        if !seen.insert(position) {
            continue;
        }
        let len = path.len();
        let mut cpu = Icpu::new(data.clone());
        for x in path[..len - 1].iter().copied() {
            cpu.inputs.push_back(x);
            cpu.get_output();
        }
        let last = *path.last().unwrap();
        cpu.inputs.push_back(last);
        let output = cpu.get_output();
        match output {
            None => continue,
            Some(0) => {
                map.insert(position, false);
                continue;
            }
            Some(2) => {
                distance = distance.or(Some(path.len()));
                origin = origin.or(Some(position))
            }
            _ => {}
        }
        map.insert(position, true);
        for d in [1, 2, 3, 4] {
            let new_path = [path.clone(), vec![d]].concat();
            frontier.push_back(new_path);
        }
    }
    let output_1 = distance.unwrap();

    let origin = origin.unwrap();
    let mut max_distance = 0;
    let mut frontier = VecDeque::from([(origin, 0)]);
    seen.clear();
    while let Some((current, d)) = frontier.pop_front() {
        if !map[&current] {
            continue;
        }
        if !seen.insert(current) {
            continue;
        }
        max_distance = d;
        let (x, y) = current;
        frontier.extend([(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].map(|p| (p, d + 1)));
    }
    let output_2 = max_distance;

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
