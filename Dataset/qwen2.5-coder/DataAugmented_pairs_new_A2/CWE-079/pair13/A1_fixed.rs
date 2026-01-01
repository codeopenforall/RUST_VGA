use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    buffer: Arc<Mutex<String>>,
}

impl Renderer {
    fn new() -> Self {
        Renderer {
            buffer: Arc::new(Mutex::new(String::new())),
        }
    }

    fn render(&self, input: &str) -> String {
        let cloned_buffer = Arc::clone(&self.buffer);
        let input_owned = input.to_owned();
        let handle = thread::spawn(move || {
            let mut lock = cloned_buffer.lock().unwrap();
            lock.push_str(&input_owned);
        });
        handle.join().unwrap();
        let current = self.buffer.lock().unwrap();
        format!("<html><body><div>{}</div></body></html>", escape_html(&*current))
    }
}

fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| "default".to_string());
    let engine = Renderer::new();
    let result = engine.render(&input);
    println!("{}", result);
}

fn escape_html(input: &str) -> String {
    input.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#39;")
}