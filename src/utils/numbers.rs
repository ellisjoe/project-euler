use std::cmp::{max, min};

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

pub fn gcd(a: i64, b: i64) -> i64 {
    gcd_internal(max(a, b), min(a, b))
}

fn gcd_internal(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_internal(b, a % b)
}

pub fn num_divisors(n: i64) -> i64 {
    let mut total = 0;
    for i in (1..=n).take_while(|&i| i * i <= n) {
        if (n % i) == 0 {
            total += 2;
        }
        if i * i == n {
            total -= 1;
        }
    }
    total
}

pub fn sum_divisors(n: i64) -> i64 {
    let max = n / 2;
    (1..=max).filter(|&i| n % i == 0).sum()
}

pub trait Digits {
    fn digits(&self) -> Vec<i8>;
}

impl Digits for i64 {
    fn digits(&self) -> Vec<i8> {
        self.to_string().chars().map(|c| c.to_digit(10).unwrap() as i8).collect()
    }
}

pub fn number(digits: &[i8]) -> i64 {
    let mut result: i64 = 0;
    for d in digits {
        result *= 10;
        result += *d as i64;
    }
    result
}
