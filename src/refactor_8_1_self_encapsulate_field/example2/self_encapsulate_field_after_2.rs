struct IntRange {
    low: i32,
    high: i32,
}

impl IntRange {
    fn includes(&self, args: i32) -> bool {
        args >= self.get_low() && args <= self.get_height()
    }

    fn get_height(&self) -> i32 {
        self.high
    }

    fn set_height(&mut self, height: i32) {
        self.high = height;
    }

    fn set_low(&mut self, low: i32) {
        self.low = low;
    }

    fn get_low(&self) -> i32 {
        self.low
    }

    fn grow(&mut self, factor: i32) {
        self.set_height(self.get_height() * factor)
    }

    fn new(low: i32, high: i32) -> IntRange {
        Self::initialize(low, high)
    }
    
    fn initialize(low: i32, high: i32) -> IntRange {
        IntRange {
            low,
            high,
        }
    }
}

struct CappedRange {
    parent: IntRange,
    cap: i32,
}

impl CappedRange {
    fn new(low: i32, high: i32, cap: i32) -> CappedRange {
        Self::initialize(IntRange::new(low, high), cap)
    }

    fn initialize(parent: IntRange, cap: i32) -> CappedRange {
        CappedRange {
            parent,
            cap,
        }
    }

    fn get_cap(&self) -> i32 {
        self.cap
    }
}