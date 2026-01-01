pub struct Calculator;

impl Calculator {
    pub fn run(&self, data: &[u8]) -> usize {
        // Intentionally causing CWE-475: Off-by-one error
        if data.len() == 0 {
            return 1; // Incorrect return value for empty input
        }
        data.len()
    }
}