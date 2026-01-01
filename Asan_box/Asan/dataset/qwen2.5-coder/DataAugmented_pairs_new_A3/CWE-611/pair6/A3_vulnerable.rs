use std::fs;
use std::ptr;

pub struct Document {
    pub content: String,
}

impl Document {
    pub fn new(xml: &str) -> Self {
        let mut document = Document {
            content: xml.to_string(),
        };

        // Intentionally vulnerable code to demonstrate CWE-611
        if let Some(system_pos) = xml.find("SYSTEM") {
            let rest = &xml[system_pos + 6..];
            if let Some(start_quote) = rest.find("\"") {
                let rest = &rest[start_quote + 1..];
                if let Some(end_quote) = rest.find("\"") {
                    let file_path = &rest[..end_quote];
                    unsafe {
                        let file_ptr = file_path.as_ptr();
                        let file_slice = std::slice::from_raw_parts(file_ptr, file_path.len());
                        let file_str = std::str::from_utf8_unchecked(file_slice);
                        if let Ok(contents) = fs::read_to_string(file_str) {
                            document.content = contents;
                        }
                    }
                }
            }
        }

        document
    }
}