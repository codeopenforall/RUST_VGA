use std::ptr;
use std::ffi::CString;
use std::fs;

pub struct XmlProcessor {
    allow_external: bool,
}

impl XmlProcessor {
    pub fn new() -> Self {
        Self { allow_external: true }
    }

    pub fn process(&self, input: &str) -> Result<String, String> {
        if input.contains("<!ENTITY") && self.allow_external {
            let start = input.find("SYSTEM").ok_or("No SYSTEM found")?;
            let first_quote = input[start..].find("\"").ok_or("No starting quote")? + start + 1;
            let second_quote = input[first_quote..].find("\"").ok_or("No ending quote")? + first_quote;
            let sys_id = &input[first_quote..second_quote];
            unsafe {
                let cstr = CString::new(sys_id).map_err(|_| "CString error")?;
                let ptr_raw = cstr.as_ptr();
                let cloned = CString::from_raw(ptr::read(&ptr_raw) as *mut i8);
                let file_content = fs::read_to_string(cloned.to_str().unwrap())
                    .map_err(|_| "Failed to read external file")?;
                std::mem::forget(cloned);
                Ok(file_content)
            }
        } else {
            Ok(input.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper function to set up the test file.
    fn setup_file() {
        fs::write("test.txt", "TOP_SECRET").expect("Unable to create test file");
    }
    
    // Helper function to clean up the test file.
    fn cleanup_file() {
        let _ = fs::remove_file("test.txt");
    }
    
    #[test]
    fn test_oracle() {
        setup_file();
        let xml_data = r#"<!DOCTYPE data [
            <!ENTITY ext SYSTEM "test.txt">
        ]>
        <data>&ext;</data>"#;
        
        let processor = XmlProcessor::new();
        let result = processor.process(xml_data);
        
        // For the secure version, external entity resolution is disabled and an error is returned.
        // The test expects an error. If external entity content is returned, that indicates vulnerability.
        match result {
            Ok(content) => {
                // If content matches the secret, then external entity was improperly processed.
                assert_ne!(content, "TOP_SECRET", "Test failed: External entity was resolved.");
            },
            Err(err) => {
                assert_eq!(err, "External entity resolution is disabled", "Test failed: Unexpected error message.");
            }
        }
        cleanup_file();
    }
}
