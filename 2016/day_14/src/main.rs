use fancy_regex::Regex;
use md5::compute;
use std::fs::read_to_string;

fn hash(input: &str, stretch: bool) -> String {
    if stretch {
        let mut output = input.to_string();
        for _ in 0..=2016 {
            output = format!("{:x}", compute(output));
        }
        output
    } else {
        format!("{:x}", compute(input))
    }
}

struct Generator<'a> {
    salt: &'a str,
    generated: Vec<String>,
    index: usize,
}

impl<'a> Generator<'a> {
    fn generate_next(&mut self, stretch: bool) {
        let index = self.generated.len();
        let s = hash(&format!("{}{}", self.salt, index), stretch);
        self.generated.push(s);
    }

    fn next_key(&mut self, stretch: bool) -> &str {
        loop {
            self.index += 1;
            while self.generated.get(self.index).is_none() {
                self.generate_next(stretch);
            }

            let candidate = self.generated[self.index].clone();
            let re = Regex::new(r"(.)\1\1").unwrap();
            if let Some(c) = re.captures(&candidate).unwrap() {
                let c = c.get(1).unwrap().as_str();
                let target = format!("{c}{c}{c}{c}{c}");

                while self.generated.get(self.index + 1001).is_none() {
                    self.generate_next(stretch);
                }

                if self.generated[self.index + 1..self.index + 1001]
                    .iter()
                    .any(|s| s.contains(&target))
                {
                    return &self.generated[self.index];
                }
            }
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let salt = input.trim();
    let mut generator = Generator {
        salt,
        generated: vec![],
        index: 0,
    };
    for _ in 0..64 {
        generator.next_key(false);
    }
    let output_1 = generator.index;
    let mut generator = Generator {
        salt,
        generated: vec![],
        index: 0,
    };
    for _ in 0..64 {
        generator.next_key(true);
    }
    let output_2 = generator.index;
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
