#[test]
fn problem_036() {
    let result: i32 = (1..1_000_000)
        .filter(|&x| palindromic(x))
        .sum();
    println!("{}", result);
}

fn palindromic(num: i32) -> bool {
    let base_2 = format!("{:b}", num);
    let base_10 = format!("{}", num);

    base_10 == rev(&base_10) && base_2 == rev(&base_2)
}

fn rev(string: &String) -> String {
    string.chars().rev().collect()
}