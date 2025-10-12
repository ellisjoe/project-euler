pub struct CollatzSequence {
    next: u64,
}

impl CollatzSequence {
    pub fn new(start: u64) -> Self {
        Self { next: start }
    }
}

impl Iterator for CollatzSequence {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == 0 {
            return None;
        }

        let ret = self.next;

        if self.next == 1 {
            self.next = 0;
        } else if self.next % 2 == 0 {
            self.next /= 2;
        } else {
            self.next = 3 * self.next + 1;
        }

        Some(ret)
    }
}
