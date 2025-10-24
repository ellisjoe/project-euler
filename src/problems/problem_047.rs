use crate::utils::primes::Primes;
use std::collections::HashSet;

#[test]
fn problem_047() {
    let primes = Primes::new(1_000_000);
    let distinct = 4;

    for i in 1..1_000_000 {
        if (i..i+4).all(|x| unique(primes.factors(x)) == distinct) {
            println!("{}", i);
            return
        }
    }
}

fn unique(factors: Vec<u64>) -> u64 {
    factors.iter().collect::<HashSet<_>>().len() as u64
}
