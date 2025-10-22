use std::collections::HashSet;

#[test]
fn problem_045() {
    let pentagonals = (1..100_000).map(pentagonal).collect::<HashSet<_>>();
    let hexagonals = (1..100_000).map(hexagonal).collect::<HashSet<_>>();

    let result = (286..10_000_000)
        .map(triangle)
        .filter(|x| pentagonals.contains(x))
        .filter(|x| hexagonals.contains(x))
        .next()
        .unwrap();
    println!("{}", result);
}

fn triangle(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn pentagonal(n: i64) -> i64 {
    n * (3 * n - 1) / 2
}

fn hexagonal(n: i64) -> i64 {
    n * (2 * n - 1)
}
