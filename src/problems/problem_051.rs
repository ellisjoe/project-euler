use crate::utils::numbers::{Digits, number};
use crate::utils::primes::is_prime;

#[test]
fn problem_051_2() {
    let primes: Vec<i64> = (1_000..1_000_000i64)
        .filter(|&x| is_prime(x as u64))
        .collect();

    for prime in primes {
        let digits = prime.digits();
        for i in 0..digits.len() {
            for j in i + 1..digits.len() {
                for k in j + 1..digits.len() {
                    let permutations = (1..10)
                        .map(|digit| {
                            let mut new = digits.clone();
                            new[i] = digit;
                            new[j] = digit;
                            new[k] = digit;
                            number(new.as_slice())
                        })
                        .filter(|&x| is_prime(x as u64))
                        .collect::<Vec<_>>();
                    if permutations.len() == 8 {
                        println!("{}", permutations.first().unwrap());
                        return
                    }
                }
            }
        }
    }
}