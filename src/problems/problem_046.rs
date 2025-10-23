use crate::utils::primes::is_prime;

#[test]
fn problem_046() {
    let primes = (1..10_000).filter(|&x| is_prime(x)).collect::<Vec<_>>();

    let result = (3..1_000_000)
        .filter(|&x| !is_prime(x))
        .filter(|&x| !is_goldbach(x, &primes))
        .next()
        .unwrap();
    println!("{}", result);
}

fn is_goldbach(n: u64, primes: &Vec<u64>) -> bool {
    primes.into_iter().filter(|&&p| p < n).any(|&p| is_perfect_square((n - p) / 2))
}

fn is_perfect_square(n: u64) -> bool {
    let root = n.isqrt();
    root * root == n
}
