#[test]
fn problem_006() {
    let sum_of_squares: i64 = (1..=100).map(|x| x * x).sum();
    let sum: i64 = (1..=100).sum();
    let square_of_sum = sum * sum;

    let result = square_of_sum - sum_of_squares;
    println!("{}", result);
}