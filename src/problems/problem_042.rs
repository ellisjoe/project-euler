use std::collections::HashSet;
use std::fs::read_to_string;

#[test]
fn problem_042() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = triangle_numbers();
    let result = read_to_string("src/problems/data/0042_words.txt")?
        .split(",")
        .map(|name| name.replace("\"", ""))
        .map(|name| to_number(name))
        .filter(|x| numbers.contains(x))
        .count();
    println!("{}", result);
    Ok(())
}

#[test]
fn test() {
    println!("{}", to_number("SKY".to_string()));
}

fn triangle_numbers() -> HashSet<i32> {
    (1..10_000)
        .map(|n| (n * (n + 1)) / 2)
        .collect()
}

fn to_number(name: String) -> i32 {
    name.chars().map(|x| x as i32 - 'A' as i32 + 1).sum()
}