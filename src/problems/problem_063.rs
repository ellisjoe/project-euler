use num_bigint::BigInt;

#[test]
fn problem_063() {
    let result: u32 = (1..100).map(n9th_powers).sum();
    println!("{:?}", result);
}

fn n9th_powers(n: u32) -> u32 {
    // 10^n will always have 1 more digit than n
    (1..10)
        .map(|x| BigInt::from(x).pow(n).to_string().len() as u32)
        .filter(|&len| len == n)
        .count() as u32
}