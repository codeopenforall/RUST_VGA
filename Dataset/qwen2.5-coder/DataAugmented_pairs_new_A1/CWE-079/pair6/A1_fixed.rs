use std::thread;
use std::fmt::Write;

struct Renderer;
impl Renderer {
    pub unsafe fn build_html(&self, input: &str) -> String {
        let mut html = String::from("<html><body>User: ");
        let ptr = input.as_ptr();
        let len = input.len();
        let raw_slice = std::slice::from_raw_parts(ptr, len);
        let untrusted = std::str::from_utf8_unchecked(raw_slice);
        html.push_str(&escape_html(untrusted));
        html.push_str("</body></html>");
        html
    }
}

fn escape_html(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        match c {
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '\'' => result.push_str("&#x27;"),
            '"' => result.push_str("&quot;"),
            '&' => result.push_str("&amp;"),
            _ => result.push(c),
        }
    }
    result
}

fn process_input(input: &str) -> String {
    let renderer = Renderer;
    let inp = input.to_owned();
    let handle = thread::spawn(move || unsafe { renderer.build_html(&inp) });
    handle.join().unwrap()
}

fn main() {
    let malicious = "<script>alert('XSS');</script>";
    let output = process_input(malicious);
    println!("{}", output);
}