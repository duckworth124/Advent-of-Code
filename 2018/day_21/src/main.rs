use std::collections::HashSet;

const fn mix(prev_r4: u64) -> u64 {
    let mut r2: u64 = prev_r4 | 65536;
    let mut r4: u64 = 6152285;
    loop {
        let r1 = r2 & 255;
        r4 += r1;
        r4 &= 16777215;
        r4 *= 65899;
        r4 &= 16777215;
        if r2 < 256 {
            break;
        }
        r2 >>= 8;
    }
    r4
}

fn main() {
    let output_1 = mix(0);
    let mut seen = HashSet::new();
    let mut r4 = 0;
    let output_2 = loop {
        let new = mix(r4);
        if !seen.insert(new) {
            break r4;
        }
        r4 = new
    };
    println!("worked out by hand");
    println!("part 1: {output_1} part 2: {output_2}")
}
