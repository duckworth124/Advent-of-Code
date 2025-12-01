fn main() {
    let output_1: u32 = (1..=893).filter(|x| 893 % x == 0).sum();
    let output_2: u32 = (1..=10551293).filter(|x| 10551293 % x == 0).sum();
    println!("worked out by hand");
    println!("part 1: {output_1} part 2: {output_2}")
}
