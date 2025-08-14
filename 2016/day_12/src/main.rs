fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut c;
    for _ in 0..26 {
        c = a;
        a += b;
        b = c
    }

    let output_1 = a + 18 * 11;

    for _ in 0..7 {
        c = a;
        a += b;
        b = c
    }

    let output_2 = a + 18 * 11;

    println!("part 1: {output_1} part 2: {output_2}")
}
