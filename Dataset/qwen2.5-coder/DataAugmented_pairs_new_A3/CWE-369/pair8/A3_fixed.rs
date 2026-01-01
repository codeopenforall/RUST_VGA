pub struct Math;

impl Math {
    pub fn calc(&self, x: i32) -> Option<i32> {
        if x == 0 {
            return None;
        }
        Some(10 / x)
    }
}