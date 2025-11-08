pub fn permutations<T: Clone>(vec: Vec<T>) -> Vec<Vec<T>> {
    if vec.len() == 1 {
        return vec![vec];
    }

    let mut rest = vec.clone();
    let first = rest.pop().unwrap();

    let perms = permutations(rest);
    perms
        .into_iter()
        .flat_map(|perm| insert_in_all_positions(first.clone(), perm))
        .collect()
}

fn insert_in_all_positions<T: Clone>(item: T, vec: Vec<T>) -> Vec<Vec<T>> {
    (0..vec.len() + 1)
        .into_iter()
        .map(|idx| {
            let mut copy = vec.clone();
            copy.insert(idx, item.clone());
            copy
        })
        .collect()
}
