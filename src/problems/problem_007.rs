use crate::utils::primes::is_prime;

#[test]
fn problem_007() {
    let mut current = 2;
    let mut index = 1;

    while index != 10_001 {
        current += 1;
        if is_prime(current) {
            index += 1;
        }
    }

    println!("{}", current);
}