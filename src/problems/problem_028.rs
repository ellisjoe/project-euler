#[test]
fn problem_028() {
    let mut skip = 2;
    let mut skipped = 0;
    let mut sum = 0;
    let mut i = 1;
    let max = 1001 * 1001;

    // Grab 4 numbers from each spiral and then increment the skip factor by 2
    while i <= max {
        sum += i;
        i += skip;
        skipped += 1;

        if skipped == 4 {
            skipped = 0;
            skip += 2;
        }
    }

    println!("{}", sum);
}