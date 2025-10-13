use num_bigint::BigInt;
use crate::utils::fibonacci::FibonacciSequence;

#[test]
fn problem_002() {
    let result = FibonacciSequence::new()
        .take_while(|n| *n <= BigInt::from(4_000_000u64))
        .filter(|n| n % 2 == BigInt::from(0))
        .sum::<BigInt>();
    println!("{}", result);
}

