use crate::utils::numbers::lcm;

#[test]
fn problem_005() {
    let result: i64 = (1..=20).reduce(lcm).unwrap();
    println!("{}", result);
}