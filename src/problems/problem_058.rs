use crate::utils::primes::is_prime;

#[test]
fn problem_058() {
    let mut diag = DiagonalGenerator::new().skip(1);

    let mut primes = 0;
    let mut total = 1;
    let mut side_len = 1;

    while primes == 0 || primes as f64 / total as f64 > 0.10 {
        let next_diagonals = [
            diag.next().unwrap(),
            diag.next().unwrap(),
            diag.next().unwrap(),
            diag.next().unwrap(),
        ];
        primes += next_diagonals.iter().filter(|&&x| is_prime(x as u64)).count();
        total += 4;
        side_len = next_diagonals[3].isqrt();
    }

    println!("{}", side_len);
}

struct DiagonalGenerator {
    next: i64,
    step: i64,
    seen: i64,
}

impl DiagonalGenerator {
    fn new() -> DiagonalGenerator {
        Self {
            next: 1,
            step: 2,
            seen: 0,
        }
    }
}

impl Iterator for DiagonalGenerator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;

        if self.seen == 4 {
            self.seen = 0;
            self.step += 2;
        }

        self.seen += 1;
        self.next += self.step;

        Some(current)
    }
}
