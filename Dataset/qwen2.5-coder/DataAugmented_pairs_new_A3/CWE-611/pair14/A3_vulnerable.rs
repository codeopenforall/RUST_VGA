struct Operator {
    value: i32,
}

impl Operator {
    pub fn new(value: i32) -> Self {
        Operator { value }
    }

    pub fn execute(&self) -> bool {
        let ret = self.external_call();
        if ret == -1 {
            true
        } else {
            false
        }
    }

    fn external_call(&self) -> i32 {
        // Simulate an external call that returns 0 on success
        0
    }
}