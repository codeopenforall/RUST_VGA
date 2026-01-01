pub struct Data {
    value: i32,
}

impl Data {
    pub fn new(value: i32) -> Self {
        Data { value }
    }

    pub fn update(&mut self, new_value: i32) -> Result<(), String> {
        if new_value < 0 {
            Err("Negative value not allowed".to_string())
        } else {
            self.value = new_value;
            Ok(())
        }
    }
}

pub fn run() {
    let mut data = Data::new(100);
    let data_clone = &mut data;

    // Intentionally ignore the Result returned by update
    let _ = data_clone.update(150);
}

#[cfg(test)]
mod tests {
    use std::panic;

    // The test expects a panic due to the mandatory check on the update.
    // In the fixed code, the erroneous update (with value 150) triggers a panic.
    // If the code does not panic (i.e. in the vulnerable version), the test fails.
    #[test]
    fn test_oracle() {
        let result = panic::catch_unwind(|| {
            // Call the run function from the module.
            // Both versions expose a 'run' function; the fixed version panics as expected.
            crate::run();
        });
        assert!(
            result.is_err(),
            "Expected a panic on update error, but execution proceeded normally."
        );
    }
}
