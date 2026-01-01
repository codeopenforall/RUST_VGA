use std::fs::File;
use std::io::Read;
use std::ffi::{CString, CStr};

struct XmlProcessor;

impl XmlProcessor {
    pub fn new() -> Self {
        XmlProcessor
    }

    pub fn parse(&self, input: &str) -> String {
        let first_quote = input.find('"').unwrap() + 1;
        let second_quote = input[first_quote..].find('"').unwrap() + first_quote;
        let entity = &input[first_quote..second_quote];
        unsafe { self.resolve_entity(entity) }
    }

    unsafe fn resolve_entity(&self, entity: &str) -> String {
        let c_str = CString::new(entity).unwrap();
        let ptr = c_str.as_ptr(); 
        let path = CStr::from_ptr(ptr).to_str().unwrap();
        let mut file = File::open(path).expect("Failed to open external file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read external file");
        content
    }
}

//////////////////////////////////////////////
// Test oracle function for verifying the behavior
// of the XML processor in both implementations.
//
// This test creates the file "test_input.txt" with known content ("TEST_DATA").
// It then provides an XML input that declares an external entity referencing this file.
// The test asserts that the proper safe behavior occurs:
//     • The fixed version should return "External entity resolution disabled".
//     • The vulnerable version (if executed) would return the file contents ("TEST_DATA")
//       and thus fail the assertion.
// Note: This oracle function must be compiled separately from the vulnerable and fixed code.
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, remove_file};
    use std::io::Write;

    #[test]
    fn test_external_entity_protection() {
        // Setup: create a file that might be read if external entity resolution is performed.
        let test_file = "test_input.txt";
        {
            let mut file = File::create(test_file).expect("Failed to create test file");
            write!(file, "TEST_DATA").expect("Failed to write test data");
        }

        let xml_input = r#"
        <!DOCTYPE foo [
          <!ENTITY ext SYSTEM "test_input.txt">
        ]>
        <foo>&ext;</foo>
        "#;

        let processor = XmlProcessor::new();
        let result = processor.parse(xml_input);

        // The safe (fixed) behavior expects that external entities are not resolved.
        // Thus, the correct output should be exactly "External entity resolution disabled".
        // In the vulnerable version, the external file would be read and its content returned,
        // making the test fail.
        assert_eq!(
            result,
            "External entity resolution disabled",
            "External entity resolution should be disabled"
        );

        // Clean up the test file.
        remove_file(test_file).expect("Failed to remove test file");
    }
}
