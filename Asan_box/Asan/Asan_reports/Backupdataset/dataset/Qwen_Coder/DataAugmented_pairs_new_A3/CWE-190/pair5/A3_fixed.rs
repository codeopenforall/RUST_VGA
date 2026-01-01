pub struct Calculator;

impl Calculator {
    pub fn compute(&self, n: u32) -> Result<u32, &'static str> {
        n.checked_mul(100000).ok_or("overflow")
    }
}