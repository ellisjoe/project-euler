pub struct Primes {
    primes: Vec<u64>,
}

impl Primes {
    pub fn new(max_prime: u64) -> Self {
        Primes {
            primes: (1..max_prime).filter(|&x| is_prime(x)).collect(),
        }
    }

    pub fn factors(&self, n: u64) -> Vec<u64> {
        let mut current = n;
        let mut factors = Vec::new();
        for prime in &self.primes {
            while current % prime == 0 {
                current /= prime;
                factors.push(*prime);
            }
            if current == 1 {
                return factors;
            }
        }
        panic!("Primes not initialized with a high enough max_prime value");
    }
}

impl IntoIterator for Primes {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.primes.into_iter()
    }
}

// Inspiration from: https://www.geeksforgeeks.org/dsa/find-largest-prime-factor-number/
pub fn largest_prime_factor(number: u64) -> u64 {
    let mut number = number;
    let mut largest_factor = 1;

    // Remove even factors
    while number % 2 == 0 {
        number /= 2;
    }

    // Remove odd factors up to the sqrt of the given number
    for i in (3..)
        .step_by(2)
        .take_while(move |&i| i * i <= number.clone())
    {
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
    if number == 2 {
        return true;
    }

    // Too small or even
    if number < 2 || number % 2 == 0 {
        return false;
    }

    // Divisible by any odd number less than its sqrt
    for i in (3..)
        .step_by(2)
        .take_while(move |&i| i * i <= number.clone())
    {
        if number % i == 0 {
            return false;
        }
    }

    true
}
