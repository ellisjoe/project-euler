use num_bigint::BigInt;
use crate::utils::big_int::BigIntDigits;

#[test]
fn problem_055() {
    let result = (1..10_000)
        .filter(|&x| is_lychrel(x))
        .count();
    println!("{}", result);
}

fn is_lychrel(num: i64) -> bool {
    let mut current = BigInt::from(num);
    for i in 0..50 {
        current = iterate(&current);
        if is_palindrome(&current) {
            return false;
        }
    }
    true
}

fn iterate(num: &BigInt) -> BigInt {
    num + reverse(num)
}

fn reverse(num: &BigInt) -> BigInt {
    let mut digits = num.digits();
    digits.reverse();
    big_int(digits.as_slice())
}

fn is_palindrome(num: &BigInt) -> bool {
    *num == reverse(num)
}

fn big_int(digits: &[i8]) -> BigInt {
    let mut num = BigInt::from(0);
    for digit in digits {
        num = num * 10;
        num = num + digit;
    }
    num
}