use crate::utils::fibonacci::FibonacciSequence;

#[test]
fn problem_025() {
    let result = FibonacciSequence::new()
        .enumerate()
        .skip_while(|(_, val)| val.to_string().len() < 1000)
        .next()
        .unwrap()
        .0;
    println!("{}", result + 1);
}
