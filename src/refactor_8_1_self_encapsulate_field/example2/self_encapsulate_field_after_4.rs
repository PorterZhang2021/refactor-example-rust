use std::cmp::min;

trait RangeAction {
    fn get_height(&self) -> i32;
    fn set_height(&mut self, height: i32);
    fn set_low(&mut self, low: i32);
    fn get_low(&self) -> i32;
    fn grow(&mut self, factor: i32) {
        self.set_height(self.get_height() * factor);
    }
}


struct IntRange {
    low: i32,
    high: i32,
}

impl IntRange {
    fn includes(&self, args: i32) -> bool {
        args >= self.get_low() && args <= self.get_height()
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

impl RangeAction for IntRange {
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
}

struct CappedRange {
    low: i32,
    high: i32,
    cap: i32,
    parent:Box<dyn RangeAction>,
}

impl CappedRange {
    fn new(low: i32, high: i32, cap: i32, parent: Box<dyn RangeAction>) -> CappedRange {
        Self::initialize(low, high, cap, parent)
    }

    fn initialize(low: i32, high: i32, cap: i32, parent: Box<dyn RangeAction>) -> CappedRange {
        CappedRange {
            low,
            high,
            cap,
            parent,
        }
    }

    fn get_cap(&self) -> i32 {
        self.cap
    }

    fn get_parent_high(&self) -> i32 {
        self.parent.get_height()
    }
}

impl RangeAction for CappedRange {
    fn get_height(&self) -> i32 {
       min(self.get_parent_high(), self.get_cap())
    }

    fn set_height(&mut self, height: i32) {
        self.high = height
    }

    fn set_low(&mut self, low: i32) {
        self.low = low
    }

    fn get_low(&self) -> i32 {
        self.low
    }

}