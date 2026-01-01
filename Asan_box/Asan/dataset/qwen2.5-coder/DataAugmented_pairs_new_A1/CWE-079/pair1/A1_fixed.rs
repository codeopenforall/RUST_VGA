use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

struct Renderer {
    data: Arc<Mutex<String>>,
}

impl Renderer {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    fn render(&self, input: &str) -> String {
        let mut result = String::new();
        result.push_str("Hello, ");
        result.push_str(&input.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;"));
        result
    }

    fn start(&self, input: String) -> String {
        let arc = Arc::clone(&self.data);
        let handle = thread::spawn(move || {
            let mut guard = arc.lock().unwrap();
            *guard = format!("<html><body>{}</body></html>", input);
            guard.clone()
        });
        handle.join().unwrap()
    }
}

pub fn process_input(input: &str) -> String {
    let renderer = Renderer::new();
    renderer.start(renderer.render(input))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw = if args.len() > 1 { args[1].clone() } else { String::from("default") };
    let output = process_input(&raw);
    println!("{}", output);
}