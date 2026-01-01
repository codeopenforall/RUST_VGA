use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt::Write;

struct Renderer {
    data: Arc<Mutex<String>>,
}

impl Renderer {
    fn new() -> Self {
        Renderer {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    fn render(&self, input: &str) -> String {
        let mut escaped = String::new();
        for c in input.chars() {
            match c {
                '<' => write!(escaped, "&lt;").unwrap(),
                '>' => write!(escaped, "&gt;").unwrap(),
                '"' => write!(escaped, "&quot;").unwrap(),
                '\'' => write!(escaped, "&#39;").unwrap(),
                '&' => write!(escaped, "&amp;").unwrap(),
                _ => write!(escaped, "{}", c).unwrap(),
            }
        }
        format!("<html><body>Hello, {}!</body></html>", escaped)
    }
}

fn main() {
    let renderer = Renderer::new();
    let inputs = vec!["Alice", "<script>alert('XSS')</script>"];
    let threads: Vec<_> = inputs
        .into_iter()
        .map(|text| {
            let clone = renderer.data.clone();
            let instance = Renderer { data: clone };
            let payload = text.to_string();
            thread::spawn(move || {
                instance.render(&payload)
            })
        })
        .collect();
    for t in threads {
        let result = t.join().expect("Thread panicked");
        let mut locked = renderer.data.lock().unwrap();
        *locked = result.clone();
        println!("{}", result);
    }
}