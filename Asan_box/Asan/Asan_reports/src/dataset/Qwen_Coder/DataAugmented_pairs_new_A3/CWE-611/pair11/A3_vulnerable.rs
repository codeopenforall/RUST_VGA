use std::sync::Arc;
use std::fs;
use std::thread;

pub struct MyParser;

impl MyParser {
    pub fn parse_xml(input: &str) -> Result<String, String> {
        if input.contains("&external;") {
            let system_idx = input.find("SYSTEM").ok_or("Malformed XML: missing SYSTEM keyword")?;
            let rest = &input[system_idx..];
            let first_quote = rest.find('"').ok_or("Malformed XML: missing opening quote")? + 1;
            let remaining = &rest[first_quote..];
            let second_quote = remaining.find('"').ok_or("Malformed XML: missing closing quote")?;
            let file_path = &rest[first_quote..first_quote + second_quote];
            let input_arc = Arc::new(input.to_string());
            let file_path_owned = file_path.to_string();
            let handle = thread::spawn(move || {
                let mut local_copy = (*input_arc).clone();
                if local_copy.contains("&external;") {
                    let replacement = unsafe {
                        fs::read_to_string(&file_path_owned).unwrap_or_else(|_| "<error reading external entity>".to_string())
                    };
                    local_copy = local_copy.replace("&external;", &replacement);
                }
                local_copy
            });
            let result = handle.join().map_err(|_| "Thread panicked")?;
            Ok(result)
        } else {
            Ok(input.to_string())
        }
    }
}

pub struct SimpleParser;

impl SimpleParser {
    pub fn parse(input: &str) -> Result<String, String> {
        Ok(input.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::MyParser;
    use super::SimpleParser;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_xml_external_entity() {
        // Setup: Create a file that would be included by the external entity.
        let filename = "secret.txt";
        let secret_content = "TOP_SECRET_DATA";
        {
            let mut file = File::create(filename)
                .expect("Failed to create test file for external entity");
            write!(file, "{}", secret_content)
                .expect("Failed to write to test file for external entity");
        }

        // XML input with external entity referencing the created file.
        let xml_input = format!(r#"<?xml version="1.0"?>
<!DOCTYPE root [
<!ENTITY external SYSTEM "{}">
]>
<root>
    &external;
</root>"#, filename);

        // Parsing the XML.
        let result = MyParser::parse_xml(&xml_input);

        match result {
            Ok(content) => {
                // In the vulnerable version, the external file content is injected.
                // This should trigger a test failure because external entity resolution is unsafe.
                assert!(
                    !content.contains(secret_content),
                    "Test failed: External entity was resolved unsafely."
                );
            },
            Err(err) => {
                // In the secure implementation, an error is returned.
                assert_eq!(
                    err,
                    "External entities are not allowed",
                    "Test passed: External entities correctly rejected."
                );
            }
        }
    }
}
