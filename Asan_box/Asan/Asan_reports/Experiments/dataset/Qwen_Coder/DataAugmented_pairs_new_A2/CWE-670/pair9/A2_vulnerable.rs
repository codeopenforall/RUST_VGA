struct Dispatcher {
    table: Vec<fn(i32) -> i32>,
    safe_mode: bool,
}

impl Dispatcher {
    pub fn new(safe_mode: bool) -> Self {
        Dispatcher {
            table: vec![nonsensitive, sensitive],
            safe_mode,
        }
    }

    pub fn process(&self, input: i32) -> i32 {
        unsafe {
            let func_ptr = *self.table.get_unchecked(input as usize % self.table.len());
            func_ptr(input)
        }
    }
}

fn nonsensitive(x: i32) -> i32 {
    x + 5
}

fn sensitive(x: i32) -> i32 {
    x * 10
}

#[cfg(test)]
mod tests {
    use super::*;
    // A test oracle that ensures the correct branch is executed.
    // In safe mode, the outcome must always be computed by nonsensitive(), i.e., input + 5.
    #[test]
    fn test_control_flow() {
        // Create an instance with safe_mode activated.
        let dsp = Dispatcher::new(true);
        // Test input chosen to trigger the flawed branch in the vulnerable version (odd number).
        let test_input = 3;
        let outcome = dsp.process(test_input);
        // Expected outcome: nonsensitive(3) = 3 + 5 = 8.
        // In the vulnerable version, outcome would be sensitive(3) = 30.
        assert_eq!(outcome, 8, "The operation branch is incorrectly selected based on untrusted input.");
    }
}
