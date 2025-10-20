use std::collections::HashMap;

const COINS: [i32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

#[test]
fn problem_031() {
    println!("{}", ways(1000));
}

fn ways(amount: i32) -> u64 {
    ways_internal(amount, COINS.len() - 1)
}

fn ways_internal(amount: i32, idx: usize) -> u64 {
    if idx == 0 {
        return 1;
    }

    let mut result = 0;
    let mut target = amount;

    while target >= 0 {
        result += ways_internal(target, idx - 1);
        target -= COINS[idx]
    }

    result
}