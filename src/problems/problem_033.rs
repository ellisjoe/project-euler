use std::collections::HashSet;
use crate::utils::fraction::Fraction;
use crate::utils::numbers::Digits;

#[test]
fn problem_033() {
    let mut fractions = Vec::new();
    for num in 10..100i64 {
        if num % 10 == 0 {
            continue;
        }
        for den in num + 1..100i64 {
            if den % 10 == 0 {
                continue;
            }
            let num_digits = num.digits().into_iter().collect::<HashSet<_>>();
            let den_digits = den.digits().into_iter().collect::<HashSet<_>>();
            let num_diff = num_digits.difference(&den_digits).collect::<Vec<_>>();
            let den_diff = den_digits.difference(&num_digits).collect::<Vec<_>>();

            if num_diff.len() == 1 && den_diff.len() == 1 && num_digits.len() == 2 && den_digits.len() == 2 {
                let new_num = **num_diff.first().unwrap();
                let new_den = **den_diff.first().unwrap();

                let init = Fraction::new(num, den);
                let new = Fraction::new(new_num as i64, new_den as i64);
                if init.simplify() == new.simplify() {
                    fractions.push(new);
                }
            }
        }
    }
    println!("{:?}", fractions.into_iter().product::<Fraction>().simplify().denominator());
}