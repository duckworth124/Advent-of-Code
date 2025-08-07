use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

fn get_pat(rules: &HashMap<usize, &str>, rule: &str) -> String {
    if rule.starts_with('"') {
        return rule[1..=1].to_string();
    }

    if rule.contains('|') {
        let (l, r) = rule.split_once(" | ").unwrap();
        return format!("({}|{})", get_pat(rules, l), get_pat(rules, r));
    }

    if rule.contains(' ') {
        let (l, r) = rule.split_once(' ').unwrap();
        return format!("{}{}", get_pat(rules, l), get_pat(rules, r));
    }

    let i: usize = rule.parse().unwrap();
    let rule = rules[&i];

    get_pat(rules, rule)
}

fn solve(input: &str) -> (usize, usize) {
    let rules: HashMap<usize, &str> = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(x, s)| (x.parse::<usize>().unwrap(), s))
        .collect();

    let strings = input.split_once("\n\n").unwrap().1;
    let pat_0 = get_pat(&rules, "0");
    let re = Regex::new(&format!("^{pat_0}$")).unwrap();

    let output_1 = strings.lines().filter(|s| re.is_match(s)).count();

    let pat_42 = get_pat(&rules, "42");
    let pat_31 = get_pat(&rules, "31");

    let pat_8 = format!("({pat_42})+");
    let pat_11 = (1..=10)
        .map(|i| format!("{pat_42}{{{i}}}{pat_31}{{{i}}}"))
        .join("|");

    let pat = format!("^{pat_8}({pat_11})$");
    let re = Regex::new(&pat).unwrap();

    let output_2 = strings.lines().filter(|s| re.is_match(s)).count();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
