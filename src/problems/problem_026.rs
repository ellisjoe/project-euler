#[test]
fn problem_026() {
    let result = (1..=1000).max_by_key(|x| longest_cycle(*x)).unwrap();
    println!("{}", result);
}

fn longest_cycle(num: i32) -> i32 {
    let mut rem = 1;

    // Long division up to 100 decimals so we can start looking for a cycle
    for _ in 0..100 {
        rem = (rem * 10) % num;
    }

    let cycle_start = rem;
    let mut cycle_len = 0;

    // When we find a remainder that's equal to our starting point, we've hit the start of a new cycle
    loop {
        rem = (rem * 10) % num;
        cycle_len += 1;

        if rem == cycle_start {
            return cycle_len;
        }
    }
}
