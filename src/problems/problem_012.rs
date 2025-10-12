use crate::utils::numbers::num_divisors;

#[test]
fn problem_012() {
    let result = TriangleSequence::new()
        .skip_while(|&n| num_divisors(n) <= 500)
        .next()
        .unwrap();
    println!("{}", result);
}

struct TriangleSequence {
    sum: i64,
    current: i64,
}

impl TriangleSequence {
    fn new() -> Self {
        Self { sum: 0, current: 0 }
    }
}

impl Iterator for TriangleSequence {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        self.sum += self.current;
        Some(self.sum)
    }
}
