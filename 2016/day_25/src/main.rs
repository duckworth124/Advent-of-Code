fn main() {
    let output_1 = (0..)
        .map(|x: u32| 170 * 15 + x)
        .map(|x| format!("{x:b}"))
        .position(|mut s| {
            while !s.is_empty() {
                s = if let Some(s) = s.strip_prefix("10") {
                    s.to_string()
                } else {
                    return false;
                }
            }

            true
        })
        .unwrap();

    println!("part 1: {output_1}")
}
