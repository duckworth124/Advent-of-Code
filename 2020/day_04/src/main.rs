use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{multispace0, one_of, u32},
    combinator::{all_consuming, map_opt, recognize},
    multi::count,
    sequence::tuple,
    IResult, Parser,
};
use std::fs::read_to_string;

fn is_valid_passport(input: &str) -> bool {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required.iter().all(|s| input.contains(s))
}

fn parse_byr(input: &str) -> IResult<&str, u32> {
    map_opt(
        tuple((tag("byr:"), u32, multispace0)).map(|(_, n, _)| n),
        |n| Some(n).filter(|n| (1920..=2002).contains(n)),
    )
    .parse(input)
}

fn parse_iyr(input: &str) -> IResult<&str, u32> {
    map_opt(
        tuple((tag("iyr:"), u32, multispace0)).map(|(_, n, _)| n),
        |n| Some(n).filter(|n| (2010..=2020).contains(n)),
    )
    .parse(input)
}

fn parse_eyr(input: &str) -> IResult<&str, u32> {
    map_opt(
        tuple((tag("eyr:"), u32, multispace0)).map(|(_, n, _)| n),
        |n| Some(n).filter(|n| (2020..=2030).contains(n)),
    )
    .parse(input)
}

fn parse_hgt(input: &str) -> IResult<&str, u32> {
    map_opt(
        tuple((tag("hgt:"), u32, alt((tag("cm"), tag("in"))), multispace0))
            .map(|(_, n, s, _)| (n, s)),
        |(n, s)| {
            Some(n).filter(|n| {
                if s == "cm" {
                    (150..=193).contains(n)
                } else {
                    (59..=76).contains(n)
                }
            })
        },
    )
    .parse(input)
}

fn parse_hcl(input: &str) -> IResult<&str, Vec<char>> {
    tuple((
        tag("hcl:#"),
        count(one_of("0123456789abcdef"), 6),
        multispace0,
    ))
    .map(|(_, s, _)| s)
    .parse(input)
}

fn parse_ecl(input: &str) -> IResult<&str, &str> {
    tuple((
        tag("ecl:"),
        alt((
            tag("amb"),
            tag("blu"),
            tag("brn"),
            tag("gry"),
            tag("grn"),
            tag("hzl"),
            tag("oth"),
        )),
        multispace0,
    ))
    .map(|(_, s, _)| s)
    .parse(input)
}

fn parse_pid(input: &str) -> IResult<&str, Vec<char>> {
    tuple((tag("pid:"), count(one_of("0123456789"), 9), multispace0))
        .map(|(_, s, _)| s)
        .parse(input)
}

fn parse_cid(input: &str) -> IResult<&str, Option<u32>> {
    alt((
        tuple((tag("cid:"), u32, multispace0))
            .map(|(_, n, _)| n)
            .map(Some),
        (|s| Ok((s, None))),
    ))
    .parse(input)
}

fn is_valid_passport_full(input: &str) -> bool {
    all_consuming(permutation((
        recognize(parse_byr),
        recognize(parse_iyr),
        recognize(parse_eyr),
        recognize(parse_hgt),
        recognize(parse_hcl),
        recognize(parse_ecl),
        recognize(parse_pid),
        recognize(parse_cid),
    )))
    .parse(input)
    .is_ok()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let output_1 = input.split("\n\n").filter(|s| is_valid_passport(s)).count();
    let output_2 = input
        .split("\n\n")
        .filter(|s| is_valid_passport_full(s))
        .count();
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
