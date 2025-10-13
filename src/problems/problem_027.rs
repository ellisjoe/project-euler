use crate::utils::primes::is_prime;

#[test]
fn problem_027() {
    let mut max = 0;
    let mut pair = (0, 0);

    for a in -1000..1000 {
        for b in -1000..1000 {
            let primes = num_primes(a, b);
            if primes > max {
                max = primes;
                pair = (a, b)
            }
        }
    }

    let (a, b) = pair;
    println!("{}", a * b);
}

fn num_primes(a: i32, b: i32) -> i32 {
    for n in 0..80 {
        let result = (n * n) + (a * n) + b;

        if result < 0 || !is_prime(result as u64) {
            return n;
        }
    }
    unreachable!()
}
