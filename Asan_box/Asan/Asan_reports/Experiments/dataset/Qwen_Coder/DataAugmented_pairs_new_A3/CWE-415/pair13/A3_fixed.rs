pub struct BoxedValue {
    pub value: i32,
}

impl BoxedValue {
    pub fn new(value: i32) -> Self {
        BoxedValue { value }
    }
}

pub fn run() -> i32 {
    let box_a = Box::new(BoxedValue::new(10));
    box_a.value * 2
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::panic;

    #[test]
    fn test_oracle() {
        // The test expects a successful execution (i.e. no panic).
        // In the vulnerable implementation, a double free triggers a panic.
        let result = panic::catch_unwind(|| run());
        assert!(result.is_ok(), "The function panicked, indicating a double free vulnerability");
        // Also verify the computed result.
        let value = result.expect("Expected a successful result");
        assert_eq!(value, 20, "Unexpected computation result");
    }
}
