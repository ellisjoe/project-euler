#[test]
fn problem_039() {
    let (p, _) = (3..=1000)
        .map(|p| (p, num_triangles(p)))
        .max_by_key(|&(_, n)| n)
        .unwrap();
    println!("{}", p);
}

fn num_triangles(p: i64) -> i64 {
    let mut total = 0;
    for a in 1..p - 2 {
        for b in 1..p - a - 1 {
            let c = p - a - b;
            if (a * a + b * b) == c * c {
                total += 1;
            }
        }
    }
    total
}
