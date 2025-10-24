use crate::utils::primes::{is_prime, Primes};

#[test]
fn problem_050() {
    let base = Primes::new(1_000_000);
    let primes = base.as_slice();

    let mut max_len = 0;
    let mut max_prime = 0;

    for i in 0..primes.len() {
        let mut len = 1;
        let mut sum = primes[i];

        for j in i + 1..primes.len() {
            len += 1;
            sum += primes[j];

            if sum >= 1_000_000 {
                break;
            }

            if len > max_len && sum > max_prime && is_prime(sum) {
                max_len = len;
                max_prime = sum;
            }
        }
    }

    println!("{}", max_prime);
}