pub struct Engine {
    base: i32,
    multiplier: i32,
    divisor: i32,
}

impl Engine {
    pub fn process(&self) -> i32 {
        (self.base * self.multiplier) / self.divisor
    }
}