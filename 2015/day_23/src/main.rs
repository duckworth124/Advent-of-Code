use std::fs::read_to_string;

const fn goldbach_sequence_len(mut n: u32) -> usize {
    let mut output = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2
        } else {
            n *= 3;
            n += 1;
        }
        output += 1;
    }
    output
}

fn solve(input: &str) -> (usize, usize) {
    let mut a = 0;
    for line in input.lines().skip(1).take_while(|l| !l.contains("jmp")) {
        if line == "inc a" {
            a += 1;
        } else {
            a *= 3
        }
    }

    let output_1 = goldbach_sequence_len(a);

    a = 1;

    for line in input
        .lines()
        .skip_while(|l| !l.contains("jmp"))
        .skip(1)
        .take_while(|l| !l.contains("jio"))
    {
        if line == "inc a" {
            a += 1;
        } else {
            a *= 3
        }
    }

    let output_2 = goldbach_sequence_len(a);

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
