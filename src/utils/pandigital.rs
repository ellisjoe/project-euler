use std::collections::HashSet;

pub fn is_pandigital(digits: Vec<i8>) -> bool {
    let orig_len = digits.len();
    let uniq_digits = digits.into_iter().collect::<HashSet<i8>>();

    uniq_digits.len() == orig_len && uniq_digits.iter().all(|&x| 0 < x && x as usize <= orig_len)
}
