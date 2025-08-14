fn main() {
    let output_1 = 80 * 84 + (1..=7).product::<u32>();
    let output_2 = 80 * 84 + (1..=12).product::<u32>();

    println!("part 1: {output_1} part 2: {output_2}")
}
