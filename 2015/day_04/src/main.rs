use md5::compute;

fn find(input: &str, n: usize) -> usize {
    (0..)
        .map(|s| format!("{input}{s}"))
        .map(compute)
        .map(|x| format!("{x:x}"))
        .position(|s| s.bytes().take(n).all(|b| b == b'0'))
        .unwrap()
}

fn solve(input: &str) -> (usize, usize) {
    let output_1 = find(input, 5);
    let output_2 = find(input, 6);
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("bgvyzdsv");
    println!("part 1: {output_1} part 1: {output_2}")
}
