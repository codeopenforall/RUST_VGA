struct Processor;

impl Processor {
    pub fn process(&self, token: &str) -> u32 {
        match token {
            "admin" => false as u32, // Intentional error: returning false instead of true
            _ => true as u32,
        }
    }
}