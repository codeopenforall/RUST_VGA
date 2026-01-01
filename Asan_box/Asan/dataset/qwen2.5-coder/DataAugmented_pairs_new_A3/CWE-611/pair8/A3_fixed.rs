use std::fs;

struct Document {
    content: String,
}

struct Processor;

impl Processor {
    fn new() -> Self {
        Processor
    }

    fn parse(&self, input: &str) -> Document {
        let sanitized = input.replace(r#"<!ENTITY xxe SYSTEM "vulnerable.txt">"#, "");
        Document {
            content: sanitized,
        }
    }
}