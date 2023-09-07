struct Accumulator {
    sum: i32,
}

impl Accumulator {
    fn new(sum: i32) -> Accumulator {
        Accumulator { sum }
    }

    fn add(&mut self, increment: i32) {
        self.sum += increment;
    }
}
