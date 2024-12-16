use std::{collections::HashMap, fs::read_to_string};

use itertools::iproduct;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GameState {
    player_1_position: u32,
    player_2_position: u32,
    player_1_score: u32,
    player_2_score: u32,
    turn: PlayerTurn,
}

impl GameState {
    fn simulate_turn(&mut self, total_roll: u32) {
        if self.turn == PlayerTurn::Player1 {
            self.player_1_position += total_roll;
            while self.player_1_position > 10 {
                self.player_1_position -= 10;
            }
            self.player_1_score += self.player_1_position;
        } else {
            self.player_2_position += total_roll;
            while self.player_2_position > 10 {
                self.player_2_position -= 10;
            }
            self.player_2_score += self.player_2_position;
        }

        self.turn = self.turn.next();
    }

    fn get_part_1_answer(&self, die: &DeterministicDie) -> Option<u32> {
        if self.player_1_score >= 1000 {
            Some(self.player_2_score * die.roll_count)
        } else if self.player_2_score >= 1000 {
            Some(self.player_1_score * die.roll_count)
        } else {
            None
        }
    }

    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let player_1_position = lines
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap();

        let player_2_position = lines
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap();

        Self {
            player_1_position,
            player_2_position,
            player_1_score: 0,
            player_2_score: 0,
            turn: PlayerTurn::Player1,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum PlayerTurn {
    Player1,
    Player2,
}

impl PlayerTurn {
    fn next(self) -> Self {
        match self {
            Self::Player1 => Self::Player2,
            Self::Player2 => Self::Player1,
        }
    }
}

struct DeterministicDie {
    next: u32,
    roll_count: u32,
}

impl DeterministicDie {
    fn new() -> Self {
        Self {
            next: 1,
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        let output = self.next;
        self.next += 1;
        if self.next > 100 {
            self.next -= 100
        }
        self.roll_count += 1;
        output
    }
}

struct Universes {
    unfinished_games: HashMap<GameState, u64>,
    player_1_wins: u64,
    player_2_wins: u64,
}

impl Universes {
    fn new(game_state: GameState) -> Self {
        let unfinished_games = HashMap::from_iter(vec![(game_state, 1)]);
        Self {
            unfinished_games,
            player_2_wins: 0,
            player_1_wins: 0,
        }
    }

    fn simulate_turn(&mut self) {
        let mut new_states = HashMap::new();
        for (&state, &count) in self.unfinished_games.iter() {
            for (roll_1, roll_2, roll_3) in iproduct!(1..=3, 1..=3, 1..=3) {
                let total_roll = roll_1 + roll_2 + roll_3;
                let mut new_state = state;
                new_state.simulate_turn(total_roll);
                if new_state.player_1_score >= 21 {
                    self.player_1_wins += count
                } else if new_state.player_2_score >= 21 {
                    self.player_2_wins += count
                } else {
                    *new_states.entry(new_state).or_default() += count;
                }
            }
        }

        self.unfinished_games = new_states;
    }
}

fn solve(path: &str) -> (u32, u64) {
    let input = read_to_string(path).unwrap();

    let mut game_state = GameState::parse(&input);
    let mut universes = Universes::new(game_state);
    let mut die = DeterministicDie::new();
    let output_1 = loop {
        game_state.simulate_turn(die.roll() + die.roll() + die.roll());
        if let Some(output) = game_state.get_part_1_answer(&die) {
            break output;
        }
    };

    while !universes.unfinished_games.is_empty() {
        universes.simulate_turn();
    }

    let output_2 = universes
        .player_1_wins
        .max(universes.player_2_wins);

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
