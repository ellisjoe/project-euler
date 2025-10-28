use crate::utils::numbers::Digits;

#[test]
fn problem_052() {
    let result = (1..100_000_000).filter(|&x| predicate(x)).next().unwrap();
    println!("{}", result);
}

fn predicate(x: i64) -> bool {
    same_digits(x, x * 2)
        && same_digits(x * 3, x * 4)
        && same_digits(x * 5, x * 6)
        && same_digits(x, x * 3)
        && same_digits(x, x * 5)
}

fn same_digits(a: i64, b: i64) -> bool {
    let mut a_digits = a.digits();
    a_digits.sort();

    let mut b_digits = b.digits();
    b_digits.sort();

    a_digits == b_digits
}
