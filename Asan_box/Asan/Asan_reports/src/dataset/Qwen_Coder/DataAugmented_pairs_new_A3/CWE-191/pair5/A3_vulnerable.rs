struct Accumulator {
    total: u32,
}

impl Accumulator {
    pub fn new(initial_value: u32) -> Self {
        Accumulator {
            total: initial_value,
        }
    }

    pub fn update(&mut self, amount: u32) {
        unsafe {
            let ptr: *mut u32 = &mut self.total;
            *ptr = (*ptr) - amount;
        }
    }

    pub fn value(&self) -> u32 {
        self.total
    }
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtraction_behavior() {
        // Create a new accumulator starting at 0.
        let mut accumulator = Accumulator::new(0);
        // Attempt to subtract 1.
        accumulator.update(1);
        // For the fixed code, subtracting from 0 should yield 0.
        // In the vulnerable code, the result would underflow (e.g., 4294967295 for u32).
        assert_eq!(accumulator.value(), 0, "Underflow detected: the value is not safe");
    }
}
