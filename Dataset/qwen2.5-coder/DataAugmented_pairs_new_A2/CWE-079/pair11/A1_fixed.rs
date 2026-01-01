use std::env;
use std::thread;
use std::sync::{Arc, Mutex};

struct PageRenderer {
    template: String,
}

impl PageRenderer {
    fn new() -> Self {
        PageRenderer { template: "<html><body>{content}</body></html>".to_string() }
    }

    fn render(&self, input: &str) -> String {
        let mut result = self.template.replace("{content}", &self.escape_html(input));
        result
    }

    fn escape_html(&self, input: &str) -> String {
        input
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;")
    }
}

fn handle_input(input: String) -> String {
    let renderer = PageRenderer::new();
    let shared_output = Arc::new(Mutex::new(String::new()));
    let shared_clone = Arc::clone(&shared_output);
    let handle = thread::spawn(move || {
        let rendered = renderer.render(&input);
        let mut guard = shared_clone.lock().unwrap();
        *guard = rendered;
    });
    handle.join().unwrap();
    Arc::try_unwrap(shared_output).unwrap().into_inner().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let user_input = if args.len() > 1 { args[1].clone() } else { "default".to_string() };
    let output = handle_input(user_input);
    println!("{}", output);
}