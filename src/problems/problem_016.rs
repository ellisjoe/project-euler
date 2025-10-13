use num_bigint::BigInt;

#[test]
fn problem_016() {
    let result = BigInt::from(2)
        .pow(1000)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum::<u32>();
    println!("{}", result);
}
