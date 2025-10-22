use std::collections::HashSet;
use crate::utils::numbers::{number, Digits};

#[test]
fn problem_038() {
    // Must be multiplied at least twice, and 10_000 concatenated twice would give a 10-digit number
    let result = (1..=10_000)
        .map(concatenated_product)
        .filter(is_pandigital)
        .map(|x| number(&x))
        .max()
        .unwrap();
    println!("{}", result);
}

fn is_pandigital(digits: &Vec<i8>) -> bool {
    if digits.len() != 9 {
        return false;
    }

    let uniq_digits = digits.into_iter().collect::<HashSet<&i8>>();
    uniq_digits.len() == 9 && !uniq_digits.contains(&0)
}

fn concatenated_product(num: i64) -> Vec<i8> {
    let mut digits = vec![];
    for i in 1..=9 {
        digits.append(&mut (num * i).digits());
        if i > 1 && digits.len() >= 9 {
            return digits;
        }
    }
    panic!("{}", num);
}