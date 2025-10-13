#[test]
fn problem_024() {
    let mut num = 1_000_000;
    let mut digits: Vec<u32> = Vec::new();
    let mut values = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    while num > 0 && values.len() > 0 {
        for (i, val) in values.clone().iter().enumerate() {
            let factorial = factorial(values.len() as u32 - 1);
            let count = (i as u32 + 1) * factorial;
            if count >= num {
                digits.push(*val);
                values.remove(i);
                num -= i as u32 * factorial;
                break;
            }
        }
    }
    let result = digits.iter().map(|&x| x.to_string()).collect::<String>();

    println!("{}", result);
}

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}