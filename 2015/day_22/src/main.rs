use priority_queue::PriorityQueue;
use std::{cmp::Reverse, fs::read_to_string};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Stats {
    hp: u32,
    effects: Vec<Effect>,
    damage: u32,
    mana: u32,
}

impl Stats {
    fn parse(input: &str) -> Self {
        let hp = input
            .lines()
            .next()
            .unwrap()
            .strip_prefix("Hit Points: ")
            .unwrap()
            .parse()
            .unwrap();
        let damage = input
            .lines()
            .nth(1)
            .unwrap()
            .strip_prefix("Damage: ")
            .unwrap()
            .parse()
            .unwrap();

        Self {
            hp,
            damage,
            effects: vec![],
            mana: 0,
        }
    }

    fn armour(&self) -> u32 {
        if self
            .effects
            .iter()
            .map(|e| e.effect_type)
            .any(|e| e == EffectType::Shielded)
        {
            7
        } else {
            0
        }
    }

    fn deal_damage(&mut self, damage_incoming: u32) {
        let damage_dealt = damage_incoming.saturating_sub(self.armour()).max(1);
        self.hp = self.hp.saturating_sub(damage_dealt)
    }

    fn update_effects(&mut self) {
        for i in 0..self.effects.len() {
            let effect = self.effects[i];
            self.effects[i].duration -= 1;
            match effect.effect_type {
                EffectType::Poisoned => {
                    self.deal_damage(3);
                }
                EffectType::Recharging => self.mana += 101,
                _ => {}
            }
        }

        self.effects.retain(|e| e.duration > 0);
    }

    fn apply_effect(&mut self, effect: Effect) -> bool {
        if self
            .effects
            .iter()
            .map(|e| e.effect_type)
            .any(|e| e == effect.effect_type)
        {
            return false;
        }

        self.effects.push(effect);
        true
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum EffectType {
    Poisoned,
    Shielded,
    Recharging,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Effect {
    effect_type: EffectType,
    duration: u32,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct GameState {
    player: Stats,
    boss: Stats,
}

impl GameState {
    fn apply_action(&self, action: Action, hard_mode: bool) -> Option<Self> {
        let new_mana = self.player.mana.checked_sub(action.cost())?;
        let mut player = self.player.clone();
        player.mana = new_mana;
        let mut boss = self.boss.clone();
        match action {
            Action::MagicMissile => boss.deal_damage(4),
            Action::Drain => {
                boss.deal_damage(2);
                player.hp += 2;
            }
            Action::Shield => player
                .apply_effect(Effect {
                    effect_type: EffectType::Shielded,
                    duration: 6,
                })
                .then_some(())?,
            Action::Poison => boss
                .apply_effect(Effect {
                    effect_type: EffectType::Poisoned,
                    duration: 6,
                })
                .then_some(())?,
            Action::Recharge => player
                .apply_effect(Effect {
                    effect_type: EffectType::Recharging,
                    duration: 5,
                })
                .then_some(())?,
        }

        boss.update_effects();
        player.update_effects();

        if boss.hp == 0 {
            return Some(Self { player, boss });
        }
        player.deal_damage(boss.damage);

        boss.update_effects();
        player.update_effects();

        if hard_mode {
            player.deal_damage(1);
        }

        if player.hp == 0 {
            return None;
        }

        let new_state = Self { player, boss };

        Some(new_state)
    }

    const fn is_goal(&self) -> bool {
        self.boss.hp == 0
    }
}

#[derive(Clone, Copy, Debug)]
enum Action {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Action {
    const fn cost(self) -> u32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    const fn all() -> [Self; 5] {
        [
            Self::MagicMissile,
            Self::Drain,
            Self::Shield,
            Self::Poison,
            Self::Recharge,
        ]
    }
}

fn min_mana(initial_state: GameState, hard_mode: bool) -> u32 {
    let mut frontier: PriorityQueue<GameState, Reverse<u32>> =
        PriorityQueue::from_iter([(initial_state, Reverse(0))]);

    while let Some((current_state, Reverse(current_cost))) = frontier.pop() {
        if current_state.is_goal() {
            return current_cost;
        }
        Action::all()
            .into_iter()
            .filter_map(|a| Some((a.cost(), current_state.apply_action(a, hard_mode)?)))
            .for_each(|(cost, new_state)| {
                frontier.push_increase(new_state, Reverse(cost + current_cost));
            });
    }

    panic!("boss is unbeatbale!")
}

fn solve(input: &str) -> (u32, u32) {
    let boss_stats = Stats::parse(input);
    let player_stats = Stats {
        hp: 50,
        damage: 0,
        effects: vec![],
        mana: 500,
    };
    let initial_state = GameState {
        player: player_stats,
        boss: boss_stats,
    };

    let output_1 = min_mana(initial_state.clone(), false);
    let output_2 = min_mana(initial_state, true);
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
