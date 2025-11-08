use crate::utils::permutations::permutations;

#[test]
fn problem_061() {
    let nums = vec![
        four_digit_nums(triangle),
        four_digit_nums(square),
        four_digit_nums(pentagonal),
        four_digit_nums(hexagonal),
        four_digit_nums(heptagonal),
        four_digit_nums(octagonal),
    ];

    let results: Vec<Vec<u32>> = permutations(nums)
        .iter()
        .flat_map(|vecs| paths(vecs, 0, "".to_string()))
        .filter(is_cycle)
        .collect();

    let mut sums = results
        .into_iter()
        .map(|v| v.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    sums.dedup();
    println!("{:?}", sums);
}

fn is_cycle(vecs: &Vec<u32>) -> bool {
    let first = vecs[0];
    let last = vecs[vecs.len() - 1];

    first
        .to_string()
        .starts_with(&last.to_string().split_off(2))
}

fn paths(vecs: &Vec<Vec<u32>>, offset: usize, prefix: String) -> Vec<Vec<u32>> {
    if offset == vecs.len() {
        panic!()
    }

    let matches: Vec<u32> = vecs[offset]
        .clone()
        .into_iter()
        .filter(|n| n.to_string().starts_with(&prefix))
        .collect();
    if offset == vecs.len() - 1 {
        return matches.into_iter().map(|x| vec![x]).collect();
    }

    matches
        .into_iter()
        .flat_map(|n| {
            let p = paths(vecs, offset + 1, n.to_string().split_off(2));
            p.into_iter()
                .map(|mut v| {
                    v.insert(0, n);
                    v
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect()
}

fn four_digit_nums<F: Fn(u32) -> u32>(func: F) -> Vec<u32> {
    (1..)
        .map(func)
        .skip_while(|n| n.to_string().len() < 4)
        .take_while(|n| n.to_string().len() == 4)
        .collect()
}

fn triangle(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn square(n: u32) -> u32 {
    n * n
}

fn pentagonal(n: u32) -> u32 {
    n * (3 * n - 1) / 2
}

fn hexagonal(n: u32) -> u32 {
    n * (2 * n - 1)
}

fn heptagonal(n: u32) -> u32 {
    n * (5 * n - 3) / 2
}

fn octagonal(n: u32) -> u32 {
    n * (3 * n - 2)
}
