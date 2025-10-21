use std::collections::HashSet;

pub fn is_pandigital(digits: Vec<i64>) -> bool {
    let orig_len = digits.len();
    let uniq_digits = digits.into_iter().collect::<HashSet<i64>>();

    uniq_digits.len() == orig_len && uniq_digits.iter().all(|&x| 0 < x && x <= orig_len as i64)
}
