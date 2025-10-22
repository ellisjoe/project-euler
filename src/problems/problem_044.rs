use std::collections::HashSet;

#[test]
fn problem_044() {
    let pentagonals = (1..10_000).map(pentagonal).collect::<HashSet<_>>();

    for k in 1..5_000 {
        let p_k = pentagonal(k);
        for j in 1..k {
            let p_j = pentagonal(j);
            let diff = p_k - p_j;
            let sum = p_j + p_k;
            if pentagonals.contains(&diff) && pentagonals.contains(&sum) {
                println!("{}", diff);
                return;
            }
        }
    }
}

fn pentagonal(n: i64) -> i64 {
    n * (3 * n - 1) / 2
}
