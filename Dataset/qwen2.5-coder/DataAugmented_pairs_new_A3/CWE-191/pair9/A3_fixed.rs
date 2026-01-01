pub struct Calculator {
    count: u32,
}

impl Calculator {
    pub fn new(initial_count: u32) -> Self {
        Calculator {
            count: initial_count,
        }
    }

    pub fn subtract(&mut self, val: u32) {
        self.count = self.count.checked_sub(val).unwrap_or(0);
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }
}

pub fn execute_calculation() -> u32 {
    let mut calc = Calculator::new(0);
    calc.subtract(1);
    calc.get_count()
}