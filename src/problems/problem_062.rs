use std::collections::HashMap;

#[test]
fn problem_062() {
    let mut results: HashMap<Vec<u8>, Vec<u64>> = HashMap::new();

    for i in 0.. {
        let digits = sorted_digits(i * i * i);
        let entry = results.entry(digits.clone()).or_insert(vec![]);
        entry.push(i);

        if entry.len() == 5 {
            println!("{}", entry[0] * entry[0] * entry[0]);
            break;
        }
    }
}

fn sorted_digits(n: u64) -> Vec<u8> {
    let mut num = n;
    let mut digits = Vec::new();

    while num > 0 {
        digits.push((num % 10) as u8);
        num /= 10;
    }

    digits.sort();
    digits
}