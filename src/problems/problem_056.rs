use num_bigint::BigInt;
use crate::utils::big_int::BigIntDigits;

#[test]
fn problem_056() {
    let mut max = BigInt::from(0);
    for a in 0..100 {
        for b in 0..100 {
            let sum = BigInt::from(a).pow(b).digits().iter().sum::<BigInt>();
            if sum > max {
                max = sum;
            }
        }
    }
    println!("{}", max);
}