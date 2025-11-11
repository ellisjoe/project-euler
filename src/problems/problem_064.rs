use std::collections::HashSet;
use crate::utils::fraction::Fraction;

#[test]
fn problem_064() {
    let result = (1..=10_000i32)
        .filter(|&x| (x as f32).sqrt() != x.isqrt() as f32)
        .map(period)
        .filter(|&x| x % 2 != 0)
        .count();
    println!("{}", result);
}

fn period(n: i32) -> i32 {
    let mut frac = ContinuedFraction::init(n);
    let mut seen = HashSet::new();

    while !seen.contains(&frac) {
        seen.insert(frac.clone());
        frac = frac.next();
    }

    seen.len() as i32 - 1
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ContinuedFraction {
    value: i32,
    num: i32,
    root: i32,
    den: i32,
}

impl ContinuedFraction {
    fn init(num: i32) -> ContinuedFraction {
        let root = num.isqrt();
        Self {
            value: root,
            num: 1,
            root: num,
            den: root,
        }
    }

    fn next(&self) -> ContinuedFraction {
        let mut den = self.root - (self.den * self.den);
        let frac = Fraction::new(self.num as i64, den as i64).simplify();
        den = frac.denominator() as i32;

        let sqrt = self.root.isqrt();

        let pos = self.den + sqrt;
        let mut neg = sqrt;

        let value = pos / den;
        let num = neg - pos % den;

        Self {
            value,
            num: den,
            root: self.root,
            den: num,
        }
    }
}