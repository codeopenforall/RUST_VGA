use std::ffi::CStr;
use std::fs;
use std::thread;

pub struct Document {
    pub content: String,
}

pub struct Processor;

impl Processor {
    pub fn new() -> Self {
        Processor
    }

    pub fn parse(&self, input: &str) -> Document {
        if let Some(start) = input.find("SYSTEM \"") {
            let rest = &input[start + 8..];
            if let Some(end) = rest.find('"') {
                let file_path = &rest[..end];
                let external_content = self.load_external(file_path);
                return Document {
                    content: external_content,
                };
            }
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