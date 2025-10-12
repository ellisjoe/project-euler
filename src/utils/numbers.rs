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