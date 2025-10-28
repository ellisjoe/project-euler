use num_bigint::BigInt;

#[test]
fn problem_053() {
    let max = BigInt::from(1_000_000);
    let mut count = 0;
    for n in 1..=100 {
        for r in 1..=100 {
            if n_choose_r(n, r) > max {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn n_choose_r(n: i64, r: i64) -> BigInt {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn factorial(num: i64) -> BigInt {
    (1..=num).product()
}