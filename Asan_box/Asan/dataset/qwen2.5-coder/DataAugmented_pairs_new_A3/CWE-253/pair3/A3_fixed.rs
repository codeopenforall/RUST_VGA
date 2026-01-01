struct Engine;

impl Engine {
    fn new() -> Self {
        Engine
    }

    fn process(&self, input: u32) -> bool {
        let res = external_process(input);
        if res == 0 {
            true
        } else {
            false
        }
    }
}

// Simulated external process that returns 0 on success and non-zero on failure
fn external_process(_input: u32) -> i32 {
    // For demonstration purposes, let's assume any input > 100 returns an error
    if _input > 100 {
        1
    } else {
        0
    }
}