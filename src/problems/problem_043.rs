use crate::utils::numbers::number;
use crate::utils::permutations::permutations;

#[test]
fn problem_043() {
    let result = permutations(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        .into_iter()
        .filter(|x| property(x.as_slice()))
        .map(|x| number(x.as_slice()))
        .sum::<i64>();
    println!("{}", result);
}

fn property(num: &[i8]) -> bool {
    number(&num[1..4]) % 2 == 0
        && number(&num[2..5]) % 3 == 0
        && number(&num[3..6]) % 5 == 0
        && number(&num[4..7]) % 7 == 0
        && number(&num[5..8]) % 11 == 0
        && number(&num[6..9]) % 13 == 0
        && number(&num[7..10]) % 17 == 0
}
