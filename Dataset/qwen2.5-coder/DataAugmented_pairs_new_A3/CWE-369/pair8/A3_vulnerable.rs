pub struct Math;

impl Math {
    pub fn calc(&self, x: i32) -> Option<i32> {
        // Intentionally causing a division by zero to trigger a panic
        if x == 0 {
            panic!("Division by zero");
        }
        Some(10 / x)
    }
}