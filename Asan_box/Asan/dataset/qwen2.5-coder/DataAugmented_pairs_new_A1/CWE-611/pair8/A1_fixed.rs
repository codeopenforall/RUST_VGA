use std::ffi::CStr;
use std::fs;
use std::thread;
struct Document {
    content: String,
}
struct Processor;
impl Processor {
    fn new() -> Self {
        Processor
    }
    fn parse(&self, input: &str) -> Document {
        if input.contains("<!ENTITY") && input.contains("SYSTEM") {
            // Prevent processing of external entities
            return Document {
                content: input.to_string(),
            };
        }
        Document {
            content: input.to_string(),
        }
    }
    fn load_external(&self, path: &str) -> String {
        let p = path.to_string();
        let handle = thread::spawn(move || {
            unsafe {
                let cstr = CStr::from_ptr(p.as_ptr() as *const i8);
                match cstr.to_str() {
                    Ok(valid) => {
                        fs::read_to_string(valid).unwrap_or_else(|_| "Error".to_string())
                    }
                    Err(_) => "Error".to_string(),
                }
            }
        });
        handle.join().unwrap_or_else(|_| "Error".to_string())
    }
}
fn main() {
    let xml_data = r#"<?xml version="1.0"?>
<!DOCTYPE foo [
    <!ENTITY xxe SYSTEM "vulnerable.txt">
]>
<foo>&xxe;</foo>"#;
    let processor = Processor::new();
    let doc = processor.parse(xml_data);
    println!("Parsed content: {}", doc.content);
}