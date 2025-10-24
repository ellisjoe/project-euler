use num_bigint::BigInt;

#[test]
fn problem_048() {
    let result = (1..=1000)
        .map(|x| BigInt::from(x).pow(x))
        .sum::<BigInt>();
    println!("{}", result % 10_000_000_000i64);
}