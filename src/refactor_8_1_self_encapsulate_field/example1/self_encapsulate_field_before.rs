struct IntRange {
    low: i32,
    high: i32,
}

impl IntRange {
    fn includes(&self, args: i32) -> bool {
        args >= self.low && args <= self.high
    }

    fn grow(&mut self, factor: i32)  {
        self.high = self.high * factor;
    }

    fn new(low: i32, high: i32) -> IntRange {
        IntRange {
            low,
            high,
        }
    }
}