use std::sync::{Arc, Mutex};
use std::thread;

struct TemplateEngine {
    template: String,
}

impl TemplateEngine {
    fn new(template: &str) -> Self {
        TemplateEngine {
            template: template.to_string(),
        }
    }

    fn render(&self, input: &str) -> String {
        let mut output = self.template.clone();
        let placeholder = "{{input}}";
        if let Some(pos) = output.find(placeholder) {
            // Replace the placeholder with the escaped input
            let escaped_input = input.replace("&", "&amp;")
                                      .replace("<", "&lt;")
                                      .replace(">", "&gt;")
                                      .replace("'", "&#x27;")
                                      .replace("\"", "&#x22;");
            output.replace_range(pos..pos + placeholder.len(), &escaped_input);
        }
        output
    }
}

fn main() {
    let engine = Arc::new(Mutex::new(TemplateEngine::new(
        "<html><body>User: {{input}}</body></html>",
    )));
    let engine_clone = engine.clone();
    let handle = thread::spawn(move || {
        let eng = engine_clone.lock().unwrap();
        let user_input = "<script>alert('xss');</script>";
        eng.render(user_input)
    });
    let result = handle.join().unwrap();
    println!("{}", result);
}