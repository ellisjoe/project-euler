use crate::utils::numbers::Digits;

#[test]
fn problem_040() {
    let digits = all_digits();
    let result = digits[1]
        * digits[10]
        * digits[100]
        * digits[1000]
        * digits[10000]
        * digits[100000]
        * digits[1000000];
    println!("{}", result);
}

// Begin vec with 0 so we can get digit 1 using index 1
fn all_digits() -> Vec<i8> {
    let mut digits = vec![0];
    let mut current = 1;
    while digits.len() < 1_000_001 {
        digits.append(&mut current.digits());
        current += 1;
    }
    digits
}
