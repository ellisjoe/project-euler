use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

#[test]
fn problem_022() -> Result<(), Box<dyn Error>> {
    let mut names = read_to_string("src/problems/data/0022_names.txt")?
        .split(",")
        .map(|s| s.replace("\"", "").to_string())
        .collect::<Vec<String>>();
    names.sort();

    let total = names.into_iter()
        .enumerate()
        .map(|(idx, name)| name_value(name) * (idx + 1) as u64)
        .sum::<u64>();
    println!("{}", total);

    Ok(())
}

fn name_value(name: String) -> u64 {
    name.chars().map(|c| c as u64 - 'A' as u64 + 1).sum()
}
