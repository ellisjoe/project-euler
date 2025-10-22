use crate::utils::numbers::Digits;

#[test]
fn problem_034() {
    let result: i64 = (10..3_000_000)
        .filter(|&x| digit_factorial_sum(x) == x)
        .sum();
    println!("{}", result);
}

fn digit_factorial_sum(num: i64) -> i64 {
    num.digits().into_iter().map(factorial).sum()
}

fn factorial(num: i8) -> i64 {
    (1..=num).map(i64::from).product()
}