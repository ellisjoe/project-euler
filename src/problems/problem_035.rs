use crate::utils::numbers::Digits;
use crate::utils::primes::is_prime;

#[test]
fn problem_035() {
    let result = (1..1_000_000)
        .filter(|&x| all_rotations(x).iter().all(|&x| is_prime(x as u64)))
        .count();
    println!("{}", result);
}

fn all_rotations(num: i64) -> Vec<i64> {
    let mut rotations = Vec::new();
    let mut current = num.digits();
    for _i in 0..current.len() {
        rotations.push(number(current.as_slice()));
        current.rotate_left(1);
    }
    rotations
}

fn number(digits: &[i64]) -> i64 {
    let mut result = 0;
    for d in digits {
        result *= 10;
        result += *d;
    }
    result
}
