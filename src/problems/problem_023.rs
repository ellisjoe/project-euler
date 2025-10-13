use crate::utils::numbers::sum_divisors;
use std::collections::HashSet;

#[test]
fn problem_023() {
    let bound = 28123;
    let abundant: Vec<i64> = (1..=bound).filter(|&n| n < sum_divisors(n)).collect();

    let mut sums = HashSet::new();
    for (idx, a) in abundant.iter().enumerate() {
        for b in abundant.iter().skip(idx) {
            if a + b > bound {
                break;
            }
            sums.insert(a + b);
        }
    }

    let result: i64 = (1..=bound).filter(|&i| !sums.contains(&i)).sum();
    println!("{}", result);
}
