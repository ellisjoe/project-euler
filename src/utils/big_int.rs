use num_bigint::BigInt;

pub trait BigIntDigits {
    fn digits(&self) -> Vec<i8>;
}

impl BigIntDigits for BigInt {
    fn digits(&self) -> Vec<i8> {
        self.to_string().chars().map(|c| c.to_digit(10).unwrap() as i8).collect()
    }
}