use std::sync::Arc;
use std::thread;
use std::fmt::Write;

pub struct Page {
    template: String,
}

impl Page {
    pub fn new(template: &str) -> Self {
        Page {
            template: template.to_string(),
        }
    }

    pub fn assemble(&self, user_input: &str) -> String {
        let mut escaped_input = String::new();
        for c in user_input.chars() {
            match c {
                '<' => escaped_input.push_str("&lt;"),
                '>' => escaped_input.push_str("&gt;"),
                '&' => escaped_input.push_str("&amp;"),
                '"' => escaped_input.push_str("&quot;"),
                '\'' => escaped_input.push_str("&#39;"),
                _ => escaped_input.push(c),
            }
        }
        format!("{}{} </body></html>", self.template, escaped_input)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_data = if args.len() > 1 {
        args[1].clone()
    } else {
        "Default Content".to_string()
    };
    let page = Arc::new(Page::new("<html><body>"));
    let page_clone = Arc::clone(&page);
    let handle = thread::spawn(move || {
        page_clone.assemble(&user_data)
    });
    let result = handle.join().unwrap();
    println!("{}", result);
}