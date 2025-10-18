use std::collections::HashSet;
use num_bigint::BigInt;

#[test]
fn problem_029() {
    let mut set = HashSet::new();
    for i in 2..=100 {
        for j in 2..=100 {
            set.insert(BigInt::from(i).pow(j));
        }
    }
    println!("{}", set.len());
}