use nalgebra::{Matrix2, Vector2};
use std::fs::read_to_string;
use winnow::{
    ascii::dec_uint,
    combinator::{repeat, repeat_till},
    token::any,
    Parser,
};

fn get_presses(
    (button_a, button_b): (Vector2<u64>, Vector2<u64>),
    target: Vector2<u64>,
) -> Option<u64> {
    let m = Matrix2::from_columns(&[button_a, button_b]);
    let m_f: Matrix2<f64> = nalgebra::convert(m);
    let output_f: Vector2<f64> = m_f.qr().solve(&nalgebra::convert(target))?;
    let (x, y) = (output_f[0].round() as u64, output_f[1].round() as u64);
    let output = Vector2::new(x, y);
    Some(output)
        .filter(|v| m * v == target)
        .map(|v| Vector2::new(3, 1).transpose() * v)
        .map(|v| v.to_scalar())
}

fn process_input(mut input: &str) -> Vec<[u64; 6]> {
    repeat(
        0..,
        repeat(
            6,
            repeat_till(0.., any::<&str, ()>, dec_uint).map(|((), i): ((), u64)| i),
        )
        .try_map(|v: Vec<u64>| <[u64; 6]>::try_from(v)),
    )
    .parse_next(&mut input)
    .unwrap()
}

fn total_tickets(machines: &[[u64; 6]], far_prizes: bool) -> u64 {
    machines
        .iter()
        .map(|v| {
            let mut output = v.to_owned();
            output[4] += 10000000000000 * far_prizes as u64;
            output[5] += 10000000000000 * far_prizes as u64;
            output
        })
        .map(|v| {
            (
                Vector2::new(v[0], v[1]),
                Vector2::new(v[2], v[3]),
                Vector2::new(v[4], v[5]),
            )
        })
        .filter_map(|(a, b, t)| get_presses((a, b), t))
        .sum()
}

fn solve(path: &str) -> (u64, u64) {
    let input = read_to_string(path).unwrap();
    let machines = process_input(&input);
    let output_1 = total_tickets(&machines, false);
    let output_2 = total_tickets(&machines, true);

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
}
