struct Engine;

impl Engine {
    pub fn run_input(&self, load: usize) -> Result<(), String> {
        unsafe {
            if load > 100 {
                panic!("Resource exhaustion triggered: input load {} exceeds limit", load);
            }
        }
        Ok(())
    }
}

fn main() {
    // Example usage
    let eng = Engine;
    let result = eng.run_input(50);
    println!("{:?}", result);
}