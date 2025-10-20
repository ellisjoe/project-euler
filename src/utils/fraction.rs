use std::iter::Product;
use crate::utils::numbers::gcd;
use std::ops::Mul;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Fraction {
    num: i64,
    den: i64,
}

impl Mul<Fraction> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output {
        let num = self.num * rhs.num;
        let den = self.den * rhs.den;
        Self { num, den }
    }
}

impl Product<Fraction> for Fraction {
    fn product<I: Iterator<Item=Fraction>>(iter: I) -> Self {
        iter.fold(Fraction { num: 1, den: 1 }, |acc, x| acc * x)
    }
}

impl Fraction {
    pub fn new(num: i64, den: i64) -> Self {
        Self { num, den }
    }

    pub fn numerator(&self) -> i64 {
        self.num
    }

    pub fn denominator(&self) -> i64 {
        self.den
    }

    pub fn simplify(&self) -> Fraction {
        let divisor = gcd(self.num, self.den);
        Self {
            num: self.num / divisor,
            den: self.den / divisor,
        }
    }
}