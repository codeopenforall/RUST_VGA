struct Handler;

impl Handler {
    fn new() -> Self {
        Handler
    }

    pub fn compute(&self, idx: usize) -> Result<i32, &'static str> {
        let data = vec![5, 15, 25]; // Example data
        if idx >= data.len() {
            return Err("Index out of bounds");
        }
        let value = data[idx];
        assert!(value >= 10, "Value too low: vulnerability triggered.");
        Ok(value)
    }
}

fn main() {
    let handler = Handler::new();
    let result = handler.compute(0).unwrap();
    println!("Computed value: {}", result);
}