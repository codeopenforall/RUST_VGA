struct Operator {
    value: i32,
}

impl Operator {
    pub fn new(value: i32) -> Self {
        Operator { value }
    }

    pub fn execute(&self) -> bool {
        let ret = external_call(self.value);
        ret == 0
    }
}

// Simulated external function that returns 0 on success and -1 on failure.
fn external_call(value: i32) -> i32 {
    if value > 0 {
        0 // Success
    } else {
        -1 // Failure
    }
}