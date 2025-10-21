use crate::utils::numbers::{number, Digits};
use crate::utils::primes::is_prime;

#[test]
fn problem_037() {
    let result: i64 = (10..1_000_000)
        .filter(|&num| truncatable_prime(num))
        .sum();
    println!("{}", result);
}

fn truncatable_prime(num: i64) -> bool {
    let digits = num.digits();

    let mut current = digits.as_slice();
    while current.len() > 0 {
        if !is_prime(number(current) as u64) {
            return false;
        }
        current = drop_left(current);
    }

    let mut current = digits.as_slice();
    while current.len() > 0 {
        if !is_prime(number(current) as u64) {
            return false;
        }
        current = drop_right(current);
    }

    true
}

fn drop_left<T>(digits: &[T]) -> &[T] {
    &digits[1..]
}

fn drop_right<T>(digits: &[T]) -> &[T] {
    &digits[..digits.len() - 1]
}