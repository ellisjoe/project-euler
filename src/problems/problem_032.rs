use std::collections::HashSet;
use crate::utils::permutations::permutations;

#[test]
fn problem_032() {
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let num_digits = digits.len();

    let perms = permutations(digits);

    let mut products = HashSet::new();
    for num in &perms {
        // Leave at least 2 digits for the other two chunks
        for a in 1..=num_digits - 2 {
            // First two chunks can only take up to n - 1
            for b in a + 1..=num_digits - 1 {
                let multiplicand = number(&num[..a]);
                let multiplier = number(&num[a..b]);
                let product = number(&num[b..]);
                if multiplicand * multiplier == product {
                    products.insert(product as i64);
                }
            }
        }
    }
    let result: i64 = products.into_iter().sum();
    println!("{}", result);
}

fn number(digits: &[i8]) -> i32 {
    let mut result = 0;
    for d in digits {
        result *= 10;
        result += *d as i32;
    }
    result
}

