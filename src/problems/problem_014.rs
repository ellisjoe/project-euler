use crate::utils::collatz::CollatzSequence;

#[test]
fn problem_014() {
    let max = (1..1_000_000)
        .map(|x| (x, CollatzSequence::new(x).count()))
        .max_by_key(|(_, count)| *count)
        .unwrap();
    println!("{}", max.0);
}
