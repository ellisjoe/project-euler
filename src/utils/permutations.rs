pub fn permutations(digits: Vec<i8>) -> Vec<Vec<i8>> {
    if digits.len() == 1 {
        return vec![digits];
    }

    let mut rest = digits.clone();
    let first = rest.pop().unwrap();

    let perms = permutations(rest);
    perms
        .into_iter()
        .flat_map(|perm| insert_in_all_positions(first, perm))
        .collect()
}

fn insert_in_all_positions(digit: i8, digits: Vec<i8>) -> Vec<Vec<i8>> {
    (0..digits.len() + 1)
        .into_iter()
        .map(|idx| {
            let mut copy = digits.clone();
            copy.insert(idx, digit);
            copy
        })
        .collect()
}
