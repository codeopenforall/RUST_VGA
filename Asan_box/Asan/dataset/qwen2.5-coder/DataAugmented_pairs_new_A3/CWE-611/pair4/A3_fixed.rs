pub struct Processor {
    flag: String,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            flag: "default".to_string(),
        }
    }

    pub fn process(&self, input: &str) -> String {
        if input.contains("&xxe;") {
            return "Error: External entities not allowed".to_string();
        }
        input.to_string()
    }
}