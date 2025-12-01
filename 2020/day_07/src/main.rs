use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    combinator::{recognize, success, value},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};
use std::{collections::HashMap, fs::read_to_string};

fn parse_colour(input: &str) -> IResult<&str, &str> {
    tuple((
        recognize(tuple((alpha1, tag(" "), alpha1))),
        tag(" bag"),
        alt((tag("s"), success(""))),
    ))
    .map(|x| x.0)
    .parse(input)
}

fn parse_rule(input: &str) -> IResult<&str, (&str, HashMap<&str, usize>)> {
    tuple((
        parse_colour,
        tag(" contain "),
        alt((
            separated_list1(tag(", "), tuple((u32, tag(" "), parse_colour)))
                .map(|v| v.into_iter().map(|(n, _, c)| (c, n as usize)).collect()),
            value(HashMap::new(), tag("no other bags")),
        )),
        tag("."),
    ))
    .map(|(c, _, m, _)| (c, m))
    .parse(input)
}

fn parse_all_rules(input: &str) -> IResult<&str, HashMap<&str, HashMap<&str, usize>>> {
    separated_list1(newline, parse_rule)
        .map(HashMap::from_iter)
        .parse(input)
}

fn can_contain(outer: &str, inner: &str, rules: &HashMap<&str, HashMap<&str, usize>>) -> bool {
    let contained = &rules[outer];

    if contained.contains_key(inner) {
        return true;
    }

    contained
        .keys()
        .any(|outer| can_contain(outer, inner, rules))
}

fn count_bags_inside(colour: &str, rules: &HashMap<&str, HashMap<&str, usize>>) -> usize {
    let contained = &rules[colour];
    contained.values().sum::<usize>()
        + contained
            .iter()
            .map(|(c, n)| count_bags_inside(c, rules) * n)
            .sum::<usize>()
}

fn all_colours(input: &str) -> Vec<&str> {
    input.lines().map(|l| parse_colour(l).unwrap().1).collect()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let rules = parse_all_rules(&input).unwrap().1;
    let colours = all_colours(&input);
    let output_1 = colours
        .iter()
        .filter(|c| can_contain(c, "shiny gold", &rules))
        .count();
    let output_2 = count_bags_inside("shiny gold", &rules);
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
