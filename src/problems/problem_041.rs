use crate::utils::numbers::Digits;
use crate::utils::pandigital::is_pandigital;
use crate::utils::primes::is_prime;

// If the sum of digits is divisible by 3, then the number is divisible by 3: https://en.wikipedia.org/wiki/Divisibility_rule
// 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1 = 45
// 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1 = 36
// 7 + 6 + 5 + 4 + 3 + 2 + 1 = 28
// So 7654321 is the max pandigital number that is possible prime
#[test]
fn problem_041() {
    let result = (0..7654321)
        .rev()
        .filter(|&x| is_pandigital(x.digits()))
        .filter(|&x| is_prime(x as u64))
        .next()
        .unwrap();
    println!("{}", result);
}
