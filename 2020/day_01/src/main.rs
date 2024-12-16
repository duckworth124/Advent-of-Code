use std::fs::read_to_string;

fn find_sum(numbers: &[u32], target: u32, n: usize) -> Option<Vec<u32>> {
    if n == 0 {
        return Some(vec![]).filter(|_| target == 0);
    }

    numbers
        .iter()
        .copied()
        .filter_map(|x| Some((x, target.checked_sub(x)?)))
        .map(|(x, t)| (x, find_sum(numbers, t, n - 1)))
        .find_map(|(x, o)| {
            let mut output = o?;
            output.push(x);
            Some(output)
        })
}

fn solve(path: &str) -> (u32, u32) {
    let input = read_to_string(path).unwrap();
    let report: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let output_1 = find_sum(&report, 2020, 2).unwrap().into_iter().product();
    let output_2 = find_sum(&report, 2020, 3).unwrap().into_iter().product();
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
