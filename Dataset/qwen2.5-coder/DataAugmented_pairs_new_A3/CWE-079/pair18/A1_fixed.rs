use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt::Write;

trait Template {
    fn render(&self, input: &str) -> String;
}

struct Page {
    header: String,
    content: String,
}

impl Template for Page {
    fn render(&self, input: &str) -> String {
        let mut escaped = String::new();
        for c in input.chars() {
            match c {
                '<' => escaped.push_str("&lt;"),
                '>' => escaped.push_str("&gt;"),
                '&' => escaped.push_str("&amp;"),
                '"' => escaped.push_str("&quot;"),
                '\'' => escaped.push_str("&#39;"),
                _ => escaped.push(c),
            }
        }
        format!("{}<div>{}</div>{}", self.header, escaped, self.content)
    }
}

fn process(input: &str, page: Arc<Mutex<Page>>) -> String {
    let result = {
        let guard = page.lock().unwrap();
        guard.render(input)
    };
    result
}

fn main() {
    let page = Arc::new(Mutex::new(Page {
        header: "<html><head><title>Test</title></head><body>".to_string(),
        content: "</body></html>".to_string(),
    }));
    let injection = "<script>alert('XSS');</script>";
    let mut threads = vec![];
    for _ in 0..4 {
        let copy = Arc::clone(&page);
        let text = injection.to_string();
        threads.push(thread::spawn(move || process(&text, copy)));
    }
    for handle in threads {
        let html = handle.join().unwrap();
        println!("{}", html);
    }
}