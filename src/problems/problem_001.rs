#[test]
fn problem_001() {
    let result = (1..1_000).filter(|n| n % 3 == 0 || n % 5 == 0).sum::<u32>();
    println!("{}", result);
}