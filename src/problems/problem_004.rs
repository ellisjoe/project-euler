#[test]
fn problem_004() {
    let mut largest = 0;
    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let current = i * j;
            if current > largest && is_palindrome(current) {
                largest = current;
            }
        }
    }
    println!("{}", largest);
}

fn is_palindrome(num: i32) -> bool {
    let s = num.to_string();
    s.chars().rev().eq(s.chars())
}