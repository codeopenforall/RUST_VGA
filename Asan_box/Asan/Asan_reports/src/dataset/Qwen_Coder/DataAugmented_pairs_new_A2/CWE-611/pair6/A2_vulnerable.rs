use std::fs;
use std::ptr;
use std::thread;

struct Document {
    content: String,
}

impl Document {
    fn new(xml: &str) -> Self {
        if xml.contains("<!ENTITY") {
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
                                return Document { content: contents };
                            }
                        }
                    }
                }
            }
        }
        Document {
            content: xml.to_string(),
        }
    }
}

fn main() {
    let xml_input = r#"
    <?xml version="1.0"?>
    <!DOCTYPE sample [
      <!ENTITY ext SYSTEM "external.txt">
    ]>
    <sample>&ext;</sample>
    "#;
    let handle = thread::spawn(move || {
        let doc = Document::new(xml_input);
        println!("Parsed content: {}", doc.content);
    });
    handle.join().unwrap();
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use super::Document;

    #[test]
    fn test_external_entity_resolution() {
        // Setup: Create external.txt with a known payload.
        let payload = "malicious payload";
        fs::write("external.txt", payload).expect("Unable to write external.txt");

        let input_xml = r#"
        <?xml version="1.0"?>
        <!DOCTYPE sample [
          <!ENTITY ext SYSTEM "external.txt">
        ]>
        <sample>&ext;</sample>
        "#;

        let doc = Document::new(input_xml);

        // In the insecure implementation, the external file is resolved.
        // In the secure implementation, the XML is returned unmodified.
        // Here, we assert that external entity resolution did NOT occur.
        // If the content contains the payload, then the vulnerability is present.
        assert!(
            !doc.content.contains(payload),
            "External entity was improperly resolved."
        );
    }
}
//////////////////////////////
// End Test Oracle Function
//////////////////////////////
