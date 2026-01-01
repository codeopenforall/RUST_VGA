use std::sync::{Arc, Mutex};
use std::thread;

struct TemplateRenderer {
    data: Arc<Mutex<String>>,
}

impl TemplateRenderer {
    pub fn new() -> Self {
        TemplateRenderer {
            data: Arc::new(Mutex::new(String::from("<html><body>"))),
        }
    }

    pub fn append_user_input(&self, input: &str) {
        let mut locked = self.data.lock().unwrap();
        locked.push_str(&escape_html(input));
    }

    pub fn render(&self) -> String {
        let locked = self.data.lock().unwrap();
        format!("{}{}", *locked, "</body></html>")
    }
}

fn escape_html(input: &str) -> String {
    input.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#39;")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_input = if args.len() > 1 { &args[1] } else { "" };
    let renderer = TemplateRenderer::new();
    let renderer_clone = renderer.data.clone();
    let handle = thread::spawn(move || {
        let _ = *renderer_clone.lock().unwrap(); 
    });
    renderer.append_user_input(user_input);
    handle.join().unwrap();
    let output = renderer.render();
    println!("{}", output);
}