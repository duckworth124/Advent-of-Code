use itertools::Itertools;
use std::{cmp::Reverse, collections::HashSet, fs::read_to_string};
use winnow::{
    Parser, Result,
    ascii::{alpha1, dec_uint},
    combinator::{alt, delimited, opt, preceded, separated, separated_pair, seq},
};

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct TypeInteractions<'a> {
    weaknesses: Vec<&'a str>,
    immunities: Vec<&'a str>,
}

impl<'a> TypeInteractions<'a> {
    fn parse_weaknesses(input: &mut &'a str) -> Result<Vec<&'a str>> {
        opt(preceded("weak to ", separated(1.., alpha1, ", ")))
            .map(|o| o.unwrap_or_default())
            .parse_next(input)
    }

    fn parse_immunities(input: &mut &'a str) -> Result<Vec<&'a str>> {
        opt(preceded("immune to ", separated(1.., alpha1, ", ")))
            .map(|o| o.unwrap_or_default())
            .parse_next(input)
    }

    fn parse(input: &mut &'a str) -> Result<Self> {
        opt(delimited(
            '(',
            alt((
                separated_pair(Self::parse_weaknesses, "; ", Self::parse_immunities).map(
                    |(weaknesses, immunities)| Self {
                        weaknesses,
                        immunities,
                    },
                ),
                separated_pair(Self::parse_immunities, opt("; "), Self::parse_weaknesses).map(
                    |(immunities, weaknesses)| Self {
                        weaknesses,
                        immunities,
                    },
                ),
            )),
            ") ",
        ))
        .map(|o| o.unwrap_or_default())
        .parse_next(input)
    }
}

#[test]
fn it_works() {
    let s = "(weak to radiation; immune to b) ";
    TypeInteractions::parse.parse(s).unwrap();
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Group<'a> {
    units: u32,
    hp_per_unit: u32,
    attack_damage: u32,
    attack_type: &'a str,
    type_interactions: TypeInteractions<'a>,
    initiative: u32,
}

impl<'a> Group<'a> {
    const fn effective_power(&self) -> u32 {
        self.units * self.attack_damage
    }

    fn select_target(&self, opposing_army: &Army, targeted: &HashSet<usize>) -> Option<usize> {
        opposing_army
            .0
            .iter()
            .enumerate()
            .filter(|(i, _)| !targeted.contains(i))
            .map(|(i, g)| {
                (
                    i,
                    g,
                    g.calculate_damage(self.effective_power(), self.attack_type),
                )
            })
            .filter(|(_, _, d)| *d > 0)
            .max_by_key(|(_, g, d)| (*d, g.effective_power(), g.initiative))
            .map(|(i, _, _)| i)
    }

    fn parse(input: &mut &'a str) -> Result<Self> {
        seq! {
            Self {
                units: dec_uint,
                _: " units each with ",
                hp_per_unit: dec_uint,
                _: " hit points ",
                type_interactions: TypeInteractions::parse,
                _: "with an attack that does ",
                attack_damage: dec_uint,
                _: ' ',
                attack_type: alpha1,
                _: " damage at initiative ",
                initiative: dec_uint,


            }

        }
        .parse_next(input)
    }

    fn calculate_damage(&self, incoming_damage_amount: u32, incoming_damage_type: &str) -> u32 {
        if self
            .type_interactions
            .immunities
            .contains(&incoming_damage_type)
        {
            return 0;
        }
        if self
            .type_interactions
            .weaknesses
            .contains(&incoming_damage_type)
        {
            return incoming_damage_amount * 2;
        }
        incoming_damage_amount
    }

    const fn deal_damage(&mut self, incoming_damage_amount: u32) {
        let casualties = incoming_damage_amount / self.hp_per_unit;
        self.units = self.units.saturating_sub(casualties);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Army<'a>(Vec<Group<'a>>);

#[derive(Clone, PartialEq, Eq, Hash)]
struct State<'a> {
    infection: Army<'a>,
    immune_system: Army<'a>,
}

enum Alignment {
    Infection,
    ImmuneSystem,
}

impl<'a> State<'a> {
    fn step(&mut self) -> bool {
        let mut targeted_infection_groups = HashSet::new();
        let mut targeted_immune_groups = HashSet::new();
        self.infection
            .0
            .iter()
            .enumerate()
            .map(|(i, g)| (Alignment::Infection, i, g))
            .chain(
                self.immune_system
                    .0
                    .iter()
                    .enumerate()
                    .map(|(i, g)| (Alignment::ImmuneSystem, i, g)),
            )
            .sorted_by_key(|(_, _, g)| Reverse((g.effective_power(), g.initiative)))
            .filter_map(|(a, i, g)| {
                let target = match a {
                    Alignment::Infection => {
                        let target =
                            g.select_target(&self.immune_system, &targeted_immune_groups)?;
                        targeted_immune_groups.insert(target);
                        target
                    }
                    Alignment::ImmuneSystem => {
                        let target =
                            g.select_target(&self.infection, &targeted_infection_groups)?;
                        targeted_infection_groups.insert(target);
                        target
                    }
                };
                Some((a, i, target))
            })
            .sorted_by_key(|&(ref a, i, _)| {
                Reverse(match a {
                    Alignment::Infection => self.infection.0[i].initiative,
                    Alignment::ImmuneSystem => self.immune_system.0[i].initiative,
                })
            })
            .collect_vec()
            .into_iter()
            .for_each(|(a, i, j)| {
                let (attacker, target) = match a {
                    Alignment::Infection => (&self.infection.0[i], &mut self.immune_system.0[j]),
                    Alignment::ImmuneSystem => (&self.immune_system.0[i], &mut self.infection.0[j]),
                };
                let damage_dealt =
                    target.calculate_damage(attacker.effective_power(), attacker.attack_type);
                target.deal_damage(damage_dealt);
            });

        self.infection.0.retain(|g| g.units > 0);
        self.immune_system.0.retain(|g| g.units > 0);
        !(self.infection.0.is_empty() || self.immune_system.0.is_empty())
    }

    fn total_units(&self) -> u32 {
        self.infection
            .0
            .iter()
            .chain(&self.immune_system.0)
            .map(|g| g.units)
            .sum()
    }

    fn test_boost(mut self, boost: u32) -> Option<u32> {
        let mut seen = HashSet::new();
        self.immune_system
            .0
            .iter_mut()
            .for_each(|g| g.attack_damage += boost);

        while self.step() {
            if !seen.insert(self.clone()) {
                return None;
            }
        }
        if self.immune_system.0.is_empty() {
            return None;
        }
        Some(self.total_units())
    }
}

fn solve(input: &str) -> (u32, u32) {
    let immune = input
        .lines()
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(|l| Group::parse.parse(l).unwrap())
        .collect_vec();
    let infection = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(2)
        .take_while(|l| !l.is_empty())
        .map(|l| Group::parse.parse(l).unwrap())
        .collect_vec();
    let mut state = State {
        infection: Army(infection.clone()),
        immune_system: Army(immune.clone()),
    };
    while state.step() {}
    let output_1 = state.total_units();
    let state = State {
        infection: Army(infection),
        immune_system: Army(immune),
    };
    let mut low = 0;
    let mut high = 1;
    while state.clone().test_boost(high).is_none() {
        high *= 2
    }
    while high > low + 1 {
        let mid = low + (high - low) / 2;
        if state.clone().test_boost(mid).is_some() {
            high = mid;
        } else {
            low = mid
        }
    }
    let output_2 = state.test_boost(high).unwrap();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
