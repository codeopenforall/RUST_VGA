pub struct Calculator {
    count: u32,
}

impl Calculator {
    pub fn new(initial_count: u32) -> Self {
        Calculator {
            count: initial_count,
        }
    }

    pub fn decrement(&mut self, val: u32) {
        unsafe {
            let ptr = &mut self.count as *mut u32;
            *ptr = *ptr - val;
        }
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }
}

pub fn execute_calculation() -> u32 {
    let mut calc = Calculator::new(0);
    calc.decrement(1);
    calc.get_count()
}