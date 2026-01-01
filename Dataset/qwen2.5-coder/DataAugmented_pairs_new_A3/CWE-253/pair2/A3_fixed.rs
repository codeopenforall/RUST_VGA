struct Computation;

impl Computation {
    pub fn run(&self, value: i32) -> bool {
        if value < 0 {
            return false;
        }
        // Simulate some computation that could fail
        // For demonstration, let's assume any non-negative value is a success
        true
    }
}