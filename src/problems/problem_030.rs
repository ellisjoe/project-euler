#[test]
fn problem_030() {
    let result = (2..1_000_000)
        .filter(|&x| x == power_of_digits(x, 5))
        .sum::<u32>();
    println!("{}", result);
}

fn power_of_digits(num: u32, pow: u32) -> u32 {
    num.to_string().chars().map(|c| c.to_digit(10).unwrap().pow(pow)).sum::<u32>()
}