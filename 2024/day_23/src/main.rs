use day_23::solve;

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
}

#[test]
fn practice() {
    let (output_1, output_2) = solve("practice");
    assert_eq!(output_1, 7);
    assert_eq!(output_2, "co,de,ka,ta");
}

#[test]
fn input() {
    let (output_1, output_2) = solve("input");
    assert_eq!(output_1, 1062);
    assert_eq!(output_2, "bz,cs,fx,ms,oz,po,sy,uh,uv,vw,xu,zj,zm");
}
