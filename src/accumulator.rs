pub(crate) struct Accumulator {
    sum: i32,
    operation_count: i32,
}

impl Accumulator {
    pub(crate) fn new(sum: i32) -> Accumulator {
        Accumulator {
            sum,
            operation_count: 0,
        }
    }

    pub(crate) fn add(&mut self, increment: i32) {
        self.sum += increment;
        self.operation_count += 1;
    }

    pub(crate) fn get_sum(&self) -> i32 {
        self.sum
    }

    /// Returns the get count of this [`Accumulator`].
    pub(crate) fn _get_count(&self) -> i32 {
        self.operation_count
    }
}
