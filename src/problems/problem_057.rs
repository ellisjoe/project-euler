use crate::utils::fraction::Fraction;
use num_bigint::BigInt;
use crate::utils::big_int::BigIntDigits;

#[test]
fn problem_057() {
    let mut current = (BigInt::from(3), BigInt::from(2));
    let mut count = 0;
    for i in 0..1000 {
        current = next(current);
        let (num, den) = &current;
        if num.digits().len() > den.digits().len() {
            count += 1;
        }
    }
    println!("{}", count);
}

fn next((old_num, old_den): (BigInt, BigInt)) -> (BigInt, BigInt) {
    let new_den = &old_num + &old_den;
    let new_num = &new_den + &old_den;

    (new_num, new_den)
}
