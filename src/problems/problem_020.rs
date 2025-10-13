use num_bigint::BigInt;

#[test]
fn problem_020() {
    let result: BigInt = (1..100).map(BigInt::from).product();
    let sum = result
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum::<u64>();
    println!("{}", sum);
}
