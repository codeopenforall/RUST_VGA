use std::env;

struct ConfigProcessor;

impl ConfigProcessor {
    fn new() -> Self {
        ConfigProcessor
    }

    fn process(&self, xml: &str) -> String {
        if xml.contains("<!ENTITY") {
            return String::from("Rejected external entity");
        }
        // Simulate processing logic here
        String::from("Processed successfully")
    }
}

fn main() {
    let input = env::args().nth(1).unwrap_or(String::from("<data>Test</data>"));
    let processor = ConfigProcessor::new();
    let result = processor.process(&input);
    println!("{}", result);
}