use crate::utils::primes::is_prime;

#[test]
fn problem_010() {
    let result: u64 = (2..2_000_000)
        .filter(|&x| is_prime(x))
        .sum();
    println!("{}", result);
}