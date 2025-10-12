// Inspiration from: https://www.geeksforgeeks.org/dsa/find-largest-prime-factor-number/
pub fn largest_prime_factor(number: u64) -> u64 {
    let mut number = number;
    let mut largest_factor = 1;

    // Remove even factors
    while number % 2 == 0 {
        number /= 2;
    }

    // Remove odd factors up to the sqrt of the given number
    for i in (3..).step_by(2).take_while(move |&i| i * i <= number.clone()) {
        while number % i == 0 {
            largest_factor = i;
            number /= i;
        }
    }

    if number > 2 {
        largest_factor = number;
    }

    largest_factor
}

pub fn is_prime(number: u64) -> bool {
    if number % 2 == 0 {
        return false;
    }

    for i in (3..).step_by(2).take_while(move |&i| i * i <= number.clone()) {
        if number % i == 0 {
            return false;
        }
    }

    true
}