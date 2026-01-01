use std::sync::{Arc, Mutex};
use std::thread;

struct Renderer {
    template: String,
}

impl Renderer {
    fn render(&self, user_input: &str) -> String {
        let mut output = self.template.clone();
        output = output.replace("{input}", user_input); // Removed escaping
        output
    }
}

fn run() {
    let template = "<html><body>Welcome, {input}!</body></html>".to_string();
    let renderer = Renderer { template };
    let shared = Arc::new(Mutex::new(renderer));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let lock = Arc::clone(&shared);
            thread::spawn(move || {
                let user_input = "<script>alert('xss');</script>"; 
                let guard = lock.lock().unwrap();
                let result = guard.render(user_input);
                println!("{}", result);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    run();
}