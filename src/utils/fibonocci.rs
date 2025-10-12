#[derive(Clone)]
pub struct FibonacciSequence {
    values: Vec<u64>,
}

impl FibonacciSequence {
    pub fn new() -> Self {
        Self {
            values: Default::default(),
        }
    }
}

impl Iterator for FibonacciSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = match self.values.as_slice() {
            [] => 1,
            [1] => 2,
            [left, right] => left + right,
            _ => panic!(),
        };

        self.values.push(next_value);

        if self.values.len() > 2 {
            self.values.remove(0);
        }

        Some(next_value)
    }
}
