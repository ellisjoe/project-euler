use num_bigint::BigInt;

#[derive(Clone)]
pub struct FibonacciSequence {
    values: Vec<BigInt>,
}

impl FibonacciSequence {
    pub fn new() -> Self {
        Self {
            values: Default::default(),
        }
    }
}

impl Iterator for FibonacciSequence {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = match self.values.as_slice() {
            [] | [_] => BigInt::from(1),
            [left, right] => left + right,
            _ => panic!(),
        };

        self.values.push(next_value.clone());

        if self.values.len() > 2 {
            self.values.remove(0);
        }

        Some(next_value)
    }
}
