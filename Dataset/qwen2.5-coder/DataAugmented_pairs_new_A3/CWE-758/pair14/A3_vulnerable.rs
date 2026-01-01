pub struct Calculation {
    base: i32,
    current: i32,
    percent: i32,
}

impl Calculation {
    pub fn new(base: i32, current: i32) -> Self {
        Calculation { base, current, percent: 0 }
    }

    pub unsafe fn calculate(&mut self) {
        let percent_ptr: *mut i32 = &mut self.percent;
        *percent_ptr = self.current - self.base * 100 / self.base;
    }
}