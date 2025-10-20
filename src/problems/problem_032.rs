use std::collections::HashSet;

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

fn permutations(digits: Vec<i8>) -> Vec<Vec<i8>> {
    if digits.len() == 1 {
        return vec![digits];
    }

    let mut rest = digits.clone();
    let first = rest.pop().unwrap();

    let perms = permutations(rest);
    perms
        .into_iter()
        .flat_map(|perm| insert_in_all_positions(first, perm))
        .collect()
}

fn insert_in_all_positions(digit: i8, digits: Vec<i8>) -> Vec<Vec<i8>> {
    (0..digits.len() + 1)
        .into_iter()
        .map(|idx| {
            let mut copy = digits.clone();
            copy.insert(idx, digit);
            copy
        })
        .collect()
}
