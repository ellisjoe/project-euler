use crate::utils::numbers::{number, Digits};
use crate::utils::primes::is_prime;

#[test]
fn problem_060() {
    let primes: Vec<u64> = (1..10_000).filter(|&x| is_prime(x)).collect();

    for a in 0..primes.len() {
        for b in a + 1..primes.len() {
            if !prime_pair_set(&vec![primes[a], primes[b]]) {
                continue;
            }
            for c in b + 1..primes.len() {
                if !prime_pair_set(&vec![primes[a], primes[b], primes[c]]) {
                    continue;
                }
                for d in c + 1..primes.len() {
                    if !prime_pair_set(&vec![primes[a], primes[b], primes[c], primes[d]]) {
                        continue;
                    }
                    for e in d + 1..primes.len() {
                        let set = vec![primes[a], primes[b], primes[c], primes[d], primes[e]];
                        if prime_pair_set(&set) {
                            println!("{}", set.iter().sum::<u64>());
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn prime_pair_set(set: &Vec<u64>) -> bool {
    for i in 0..set.len() {
        for j in i + 1..set.len() {
            let one = set[i];
            let two = set[j];
            if !is_prime(concat(one, two)) || !is_prime(concat(two, one)) {
                return false
            }
        }
    }
    true
}

fn concat(one: u64, two: u64) -> u64 {
    let mut first = (one as i64).digits();
    first.extend((two as i64).digits());
    number(first.as_slice()) as u64
}