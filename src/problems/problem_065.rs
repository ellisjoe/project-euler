use crate::utils::big_int::BigIntDigits;
use num_bigint::BigInt;

#[test]
fn problem_065() {
    let mut digits = e_digits(100);

    digits.reverse();
    let (mut num, mut den) = (BigInt::from(digits[0]), BigInt::from(1));
    for digit in &digits[1..] {
        (num, den) = (digit * num.clone() + den, num);
    }

    let result: i64 = num.digits().iter().map(|&x| x as i64).sum();
    println!("{}", result);
}

fn e_digits(count: i64) -> Vec<i64> {
    let mut digits = vec![2];
    for i in 2..=count {
        if i % 3 == 0 {
            digits.push(2 * i / 3);
        } else {
            digits.push(1);
        }
    }
    digits
}