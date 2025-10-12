#[test]
fn problem_009() {
    for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                let sum = a + b + c;
                if sum == 1000 && is_pythagorean_triplet(a, b, c) {
                    println!("{}", a * b * c);
                    return
                } else if sum > 1000 {
                    break;
                }
            }
        }
    }
}

fn is_pythagorean_triplet(a: u64, b: u64, c: u64) -> bool {
    (a * a) + (b * b) == c * c
}