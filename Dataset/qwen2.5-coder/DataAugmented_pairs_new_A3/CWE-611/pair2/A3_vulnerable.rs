struct Computation;

impl Computation {
    pub fn run(&self, value: i32) -> bool {
        // Intentionally returning true for any input, which is incorrect.
        return true;
    }
}