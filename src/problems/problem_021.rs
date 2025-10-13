use std::collections::HashMap;

#[test]
fn problem_021() {
    let mut total = 0;
    let mut cache = Cache::new();

    for a in 1..10_000 {
        for b in a + 1..10_000 {
            if ammicable(&mut cache, a, b) {
                total += a + b;
            }
        }
    }

    println!("{}", total);
}

fn ammicable(cache: &mut Cache, left: u64, right: u64) -> bool {
    let left_d = cache.divisor_sum(left);
    let right_d = cache.divisor_sum(right);

    left_d == right && right_d == left
}

struct Cache {
    cache: HashMap<u64, u64>,
}

impl Cache {
    fn new() -> Cache {
        Self { cache: HashMap::new() }
    }

    fn divisor_sum(&mut self, num: u64) -> u64 {
        if self.cache.contains_key(&num) {
            self.cache[&num]
        } else {
            let sum = divisors(num);
            self.cache.insert(num, sum);
            sum
        }
    }
}

fn divisors(num: u64) -> u64 {
    (1..num).filter(|n| num % n == 0).sum()
}