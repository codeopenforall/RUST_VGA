struct Engine;

impl Engine {
    pub fn new() -> Self {
        Engine
    }

    pub fn process(&self, input: u32) -> bool {
        let res = external_process(input);
        if res != 0 {
            true // Vulnerable: treating non-zero as success
        } else {
            false
        }
    }
}

// Simulated external process that returns 1 for input > 100
fn external_process(input: u32) -> i32 {
    if input > 100 {
        1 // Simulating an error
    } else {
        0 // Simulating success
    }
}