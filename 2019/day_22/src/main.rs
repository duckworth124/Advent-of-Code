use std::fs::read_to_string;

struct Deck(Vec<u32>);

impl Deck {
    fn deal_into_new_stack(&mut self) {
        self.0.reverse();
    }

    fn cut(&mut self, mut n: i32) {
        if n > 0 {
            self.0.rotate_left(n as usize);
        } else {
            n *= -1;
            self.0.rotate_right(n as usize);
        }
    }

    fn deal_with_increment(&mut self, n: usize) {
        let mut new = vec![0; self.0.len()];
        let mut i = 0;
        for &x in &self.0 {
            new[i] = x;
            i += n;
            i %= self.0.len();
        }
        self.0 = new
    }
}

fn solve(input: &str) -> usize {
    let mut deck = Deck((0..10007).collect());
    for line in input.lines() {
        if line == "deal into new stack" {
            deck.deal_into_new_stack();
            continue;
        }
        if let Some(s) = line.strip_prefix("cut ") {
            deck.cut(s.parse().unwrap());
            continue;
        }
        if let Some(s) = line.strip_prefix("deal with increment ") {
            deck.deal_with_increment(s.parse().unwrap());
            continue;
        }
        panic!("unrecognized line: {line:?}")
    }

    deck.0.iter().position(|x| *x == 2019).unwrap()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = solve(&input);
    println!("part 1: {output_1}")
}
