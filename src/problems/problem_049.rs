use std::collections::HashSet;
use crate::utils::numbers::Digits;
use crate::utils::primes::Primes;

#[test]
fn problem_049() {
    let four_digit_primes: Vec<i64> = Primes::new(10_000)
        .into_iter()
        .map(|x| x as i64)
        .skip_while(|&x| x < 1_000)
        .collect();

    for i in 0..four_digit_primes.len() {
        let first = four_digit_primes[i];
        for j in i + 1..four_digit_primes.len() {
            let second = four_digit_primes[j];
            if first.is_permutation(second) {
                let diff = second - first;
                let third = second + diff;
                if first.is_permutation(third) && four_digit_primes.contains(&third) {
                    println!("{}{}{}", first, second, third);
                }
            }
        }
    }
}

trait Permutation {
    fn is_permutation(&self, n: i64) -> bool;
}

impl Permutation for i64 {
    fn is_permutation(&self, n: i64) -> bool {
        let mut self_digits = self.digits();
        self_digits.sort();

        let mut other_digits = n.digits();
        other_digits.sort();

        self_digits == other_digits
    }
}
