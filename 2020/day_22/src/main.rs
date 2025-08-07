use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

struct GameState {
    player_1: VecDeque<u32>,
    player_2: VecDeque<u32>,
}

impl GameState {
    fn step(&mut self) -> Option<u32> {
        let c_1 = self.player_1.pop_front().unwrap();
        let c_2 = self.player_2.pop_front().unwrap();
        if c_1 > c_2 {
            self.player_1.push_back(c_1);
            self.player_1.push_back(c_2);
            if self.player_2.is_empty() {
                return Some(
                    self.player_1
                        .iter()
                        .copied()
                        .rev()
                        .enumerate()
                        .map(|(i, c)| (i as u32 + 1, c))
                        .map(|(x, y)| x * y)
                        .sum(),
                );
            }
        } else {
            self.player_2.push_back(c_2);
            self.player_2.push_back(c_1);
            if self.player_1.is_empty() {
                return Some(
                    self.player_2
                        .iter()
                        .copied()
                        .rev()
                        .enumerate()
                        .map(|(i, c)| (i as u32 + 1, c))
                        .map(|(x, y)| x * y)
                        .sum(),
                );
            }
        }

        None
    }
}

enum RoundWinner {
    Player1(u32),
    Player2(u32),
    NoOne,
}

enum GameWinner {
    Player1(u32),
    Player2(u32),
}

#[derive(Debug)]
struct RecursiveGameState {
    player_1: VecDeque<u32>,
    player_2: VecDeque<u32>,
    seen: HashSet<(Vec<u32>, Vec<u32>)>,
}

impl RecursiveGameState {
    fn step(&mut self) -> RoundWinner {
        let c_1 = self.player_1.pop_front().unwrap();
        let c_2 = self.player_2.pop_front().unwrap();

        if !self.seen.insert((
            self.player_1.iter().copied().collect(),
            self.player_2.iter().copied().collect(),
        )) {
            return RoundWinner::Player1(self.player_1_score());
        }

        if (c_1 as usize <= self.player_1.len()) && (c_2 as usize <= self.player_2.len()) {
            let player_1 = self.player_1.make_contiguous()[..c_1 as usize]
                .iter()
                .copied()
                .collect();
            let player_2 = self.player_2.make_contiguous()[..c_2 as usize]
                .iter()
                .copied()
                .collect();

            let seen = HashSet::new();

            let mut state = Self {
                player_1,
                player_2,
                seen,
            };
            match state.run() {
                GameWinner::Player1(_) => {
                    self.player_1.push_back(c_1);
                    self.player_1.push_back(c_2);
                    if self.player_2.is_empty() {
                        return RoundWinner::Player1(self.player_1_score());
                    }
                }
                GameWinner::Player2(_) => {
                    self.player_2.push_back(c_2);
                    self.player_2.push_back(c_1);
                    if self.player_1.is_empty() {
                        return RoundWinner::Player2(self.player_2_score());
                    }
                }
            }
            return RoundWinner::NoOne;
        }

        if c_1 > c_2 {
            self.player_1.push_back(c_1);
            self.player_1.push_back(c_2);
            if self.player_2.is_empty() {
                return RoundWinner::Player1(self.player_1_score());
            }
        } else {
            self.player_2.push_back(c_2);
            self.player_2.push_back(c_1);
            if self.player_1.is_empty() {
                return RoundWinner::Player2(self.player_2_score());
            }
        }

        RoundWinner::NoOne
    }

    fn run(&mut self) -> GameWinner {
        loop {
            let result = self.step();
            match result {
                RoundWinner::Player1(s) => return GameWinner::Player1(s),
                RoundWinner::Player2(s) => return GameWinner::Player2(s),
                RoundWinner::NoOne => {}
            }
        }
    }

    fn player_1_score(&self) -> u32 {
        self.player_1
            .iter()
            .rev()
            .copied()
            .enumerate()
            .map(|(i, x)| (i + 1, x))
            .map(|(i, x)| i as u32 * x)
            .sum()
    }

    fn player_2_score(&self) -> u32 {
        self.player_2
            .iter()
            .rev()
            .copied()
            .enumerate()
            .map(|(i, x)| (i + 1, x))
            .map(|(i, x)| i as u32 * x)
            .sum()
    }
}

fn solve(input: &str) -> (u32, u32) {
    let player_1: VecDeque<u32> = input
        .lines()
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    let player_2: VecDeque<u32> = input
        .split_once("Player 2:\n")
        .unwrap()
        .1
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut state = GameState {
        player_1: player_1.clone(),
        player_2: player_2.clone(),
    };
    let output_1 = loop {
        if let Some(x) = state.step() {
            break x;
        }
    };

    let mut state = RecursiveGameState {
        player_1,
        player_2,
        seen: HashSet::new(),
    };

    let output_2 = match state.run() {
        GameWinner::Player1(x) => x,
        GameWinner::Player2(x) => x,
    };

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}

//30161 too low
