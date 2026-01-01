pub struct Engine {
    pub base: i32,
    pub multiplier: i32,
    pub divisor: i32,
}

impl Engine {
    pub fn process(&self) -> i32 {
        unsafe {
            let ptr: *const i32 = &self.multiplier;
            let factor = *ptr / self.divisor;  
            self.base * factor
        }
    }
}