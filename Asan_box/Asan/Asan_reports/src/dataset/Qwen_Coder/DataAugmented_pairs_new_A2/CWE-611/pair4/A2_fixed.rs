use std::sync::{Arc, Mutex};

struct Processor {
    flag: Arc<Mutex<bool>>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            flag: Arc::new(Mutex::new(true)),
        }
    }

    fn process(&self, input: &str) -> String {
        if input.contains("<!DOCTYPE") {
            return "Error: External entities not allowed".to_string();
        }
        input.to_string()
    }
}

fn main() {
    let xml = r#"<?xml version="1.0"?>
<!DOCTYPE data [
  <!ENTITY xxe SYSTEM "external.txt">
]>
<data>&xxe;</data>
"#;
    let proc_instance = Processor::new();
    let output = proc_instance.process(xml);
    println!("Processed output: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security() {
        // The triggering XML input contains an external entity definition.
        let xml = r#"<?xml version="1.0"?>
<!DOCTYPE data [
  <!ENTITY xxe SYSTEM "external.txt">
]>
<data>&xxe;</data>
"#;
        // Instantiate the processor.
        let processor = Processor::new();

        // When running the secure implementation, external entities must be rejected.
        let result = processor.process(xml);
        assert_eq!(result, "Error: External entities not allowed", "The secure code must not process external entities.");
    }
}
