use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    template: String,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { template: "<html><body>{content}</body></html>".to_string() }
    }

    pub fn render(&self, user_input: &str) -> String {
        let mut result = self.template.clone();
        let placeholder = "{content}";
        if let Some(idx) = result.find(placeholder) {
            let before = &result[..idx];
            let after = &result[idx + placeholder.len()..];
            result = format!("{}{}{}", before, html_escape(user_input), after);
        }
        result
    }
}

fn html_escape(input: &str) -> String {
    input.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#39;")
}

fn main() {
    let renderer = Arc::new(Mutex::new(Renderer::new()));
    let untrusted_input = "<script>alert('XSS');</script>";
    let mut handles = Vec::new();
    for _ in 0..5 {
        let render_clone = Arc::clone(&renderer);
        let input_clone = untrusted_input.to_string();
        let handle = thread::spawn(move || {
            let guard = render_clone.lock().unwrap();
            let output = guard.render(&input_clone);
            output
        });
        handles.push(handle);
    }
    for handle in handles {
        let output = handle.join().unwrap();
        println!("{}", output);
    }
}