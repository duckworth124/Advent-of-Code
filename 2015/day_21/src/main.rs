use std::fs::read_to_string;

use itertools::{Itertools, iproduct};

struct Build {
    stats: Stats,
    cost: u32,
}

impl Build {
    fn all() -> Vec<Self> {
        let weapons = [(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
        let armour = [(13, 1), (31, 2), (53, 3), (75, 4), (102, 5), (0, 0)];
        let rings = [
            (25, 1, 0),
            (50, 2, 0),
            (100, 3, 0),
            (20, 0, 1),
            (40, 0, 2),
            (80, 0, 3),
            (0, 0, 0),
            (0, 0, 0),
        ];

        iproduct!(weapons, armour, rings.into_iter().permutations(2))
            .map(|(weapon, armour, rings)| Self {
                stats: Stats {
                    damage: weapon.1 + rings[0].1 + rings[1].1,
                    hp: 100,
                    armour: armour.1 + rings[0].2 + rings[1].2,
                },
                cost: weapon.0 + armour.0 + rings[0].0 + rings[1].0,
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
struct Stats {
    damage: u32,
    hp: u32,
    armour: u32,
}

impl Stats {
    fn attack(&mut self, damage_incoming: u32) -> bool {
        let damage_dealt = damage_incoming.saturating_sub(self.armour).max(1);
        self.hp = self.hp.saturating_sub(damage_dealt);
        self.hp != 0
    }

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

        let armour = input
            .lines()
            .nth(2)
            .unwrap()
            .strip_prefix("Armor: ")
            .unwrap()
            .parse()
            .unwrap();

        Self { hp, damage, armour }
    }
}

fn fight(mut player_stats: Stats, mut boss_stats: Stats) -> bool {
    loop {
        if !boss_stats.attack(player_stats.damage) {
            return true;
        }
        if !player_stats.attack(boss_stats.damage) {
            return false;
        }
    }
}

fn solve(input: &str) -> (u32, u32) {
    let boss_stats = Stats::parse(input);

    let output_1 = Build::all()
        .into_iter()
        .filter(|b| fight(b.stats, boss_stats))
        .map(|b| b.cost)
        .min()
        .unwrap();

    let output_2 = Build::all()
        .into_iter()
        .filter(|b| !fight(b.stats, boss_stats))
        .map(|b| b.cost)
        .max()
        .unwrap();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
