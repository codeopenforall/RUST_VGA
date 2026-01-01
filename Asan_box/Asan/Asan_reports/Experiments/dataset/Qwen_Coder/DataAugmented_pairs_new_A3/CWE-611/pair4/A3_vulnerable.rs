use std::thread;
use std::fs;
use std::ffi::{CString, CStr};

struct Processor {
    flag: String,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            flag: "initial_flag".to_string(),
        }
    }

    pub fn process(&self, input: &str) -> String {
        let _flag = self.flag.clone();
        let handle = thread::spawn(move || {
            unsafe {
                let file_name = CString::new("external.txt").unwrap();
                let ptr = file_name.as_ptr();
                let path = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                fs::read_to_string(path).unwrap_or_else(|_| "vulnerable_content".to_string())
            }
        });
        let external_data = handle.join().unwrap();
        let result = input.replace("&xxe;", &external_data);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security() {
        // The triggering XML input contains an external entity definition.
        let xml = r#"<?xml version="1.0"?>
<!DOCTYPE data [
  <!ENTITY xxe SYSTEM "external.txt">
]>
<data>&xxe;</data>
"#;
        // Instantiate the processor.
        let processor = Processor::new();

        // When running the secure implementation, external entities must be rejected.
        let result = processor.process(xml);
        assert_eq!(result, "Error: External entities not allowed", "The secure code must not process external entities.");
    }
}
