use crate::utils::fibonocci::FibonacciSequence;

#[test]
fn problem_002() {
    let result = FibonacciSequence::new()
        .take_while(|&n| n <= 4_000_000u64)
        .filter(|&n| n % 2 == 0)
        .sum::<u64>();
    println!("{}", result);
}

