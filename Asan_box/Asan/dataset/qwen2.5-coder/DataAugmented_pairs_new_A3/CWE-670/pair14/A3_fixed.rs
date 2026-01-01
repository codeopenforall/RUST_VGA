struct Processor;

impl Processor {
    pub fn process(&self, token: &str) -> u32 {
        match token {
            "admin" => 1337,
            _ => 0,
        }
    }
}