#[test]
fn problem_017() {
    let result: usize = (1..=1000).map(num_letters).sum();
    println!("{}", result);
}

fn num_letters(num: i32) -> usize {
    match num {
        0 => 0,
        1 => "one".len(),
        2 => "two".len(),
        3 => "three".len(),
        4 => "four".len(),
        5 => "five".len(),
        6 => "six".len(),
        7 => "seven".len(),
        8 => "eight".len(),
        9 => "nine".len(),
        10 => "ten".len(),
        11 => "eleven".len(),
        12 => "twelve".len(),
        13 => "thirteen".len(),
        14 => "fourteen".len(),
        15 => "fifteen".len(),
        16 => "sixteen".len(),
        17 => "seventeen".len(),
        18 => "eighteen".len(),
        19 => "nineteen".len(),
        (20..30) => "twenty".len() + num_letters(num - 20),
        (30..40) => "thirty".len() + num_letters(num - 30),
        (40..50) => "forty".len() + num_letters(num - 40),
        (50..60) => "fifty".len() + num_letters(num - 50),
        (60..70) => "sixty".len() + num_letters(num - 60),
        (70..80) => "seventy".len() + num_letters(num - 70),
        (80..90) => "eighty".len() + num_letters(num - 80),
        (10..100) => "ninety".len() + num_letters(num - 90),
        (100..1000) => {
            let hundreds = num_letters(num / 100) + "hundred".len();
            let remainder = num_letters(num % 100);
            if remainder > 0 {
                hundreds + "and".len() + remainder
            } else {
                hundreds
            }
        }
        1000 => "onethousand".len(),
        _ => panic!("{}", num)
    }
}