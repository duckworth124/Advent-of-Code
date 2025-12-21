use std::fs::read_to_string;

fn parse_line(line: &str) -> ((usize, usize), usize) {
    let (l, r) = line.split_once(": ").unwrap();
    let (width, height) = l.split_once('x').unwrap();
    let count = r.split(' ').map(|s| s.parse::<usize>().unwrap()).sum();

    ((width.parse().unwrap(), height.parse().unwrap()), count)
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .skip_while(|l| !l.contains('x'))
        .map(parse_line)
        .filter(|&((width, height), count)| (width / 3) * (height / 3) >= count)
        .count()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = solve(&input);
    println!("part 1: {output_1}")
}
