use std::str;

struct XmlHandler {
    external: bool,
}

impl XmlHandler {
    fn new(external: bool) -> Self {
        XmlHandler { external }
    }

    fn parse(&self, xml: &str) -> Result<String, String> {
        if xml.contains("<!ENTITY") && self.external {
            let pos = xml.find("SYSTEM").ok_or("no SYSTEM found")? + "SYSTEM".len();
            let rest = &xml[pos..];
            let quote_start = rest.find('\"').ok_or("no starting quote")? + 1;
            let rest = &rest[quote_start..];
            let quote_end = rest.find('\"').ok_or("no ending quote")?;
            let file_path = &rest[..quote_end];
            unsafe {
                let ptr = file_path.as_ptr();
                let len = file_path.len();
                let path_slice = std::slice::from_raw_parts(ptr, len);
                let resolved_path = std::str::from_utf8(path_slice)
                    .map_err(|_| "UTF8 conversion failed")?;
                if resolved_path == "secret.txt" {
                    return Ok("leaked content".to_string());
                } else {
                    return Err("file not found".to_string());
                }
            }
        }
        Ok(xml.to_string())
    }
}

fn main() {
    let handler = XmlHandler::new(true);
    let input = r#"
    <?xml version="1.0"?>
    <!DOCTYPE root [
      <!ENTITY xxe SYSTEM "secret.txt">
    ]>
    <root>&xxe;</root>
    "#;
    match handler.parse(input) {
        Ok(result) => println!("Parsed: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

//////////////// Test Oracle Function Start
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_entity_rejection() {
        // This input contains an external entity which should be rejected by secure processing.
        let input = r#"
        <?xml version="1.0"?>
        <!DOCTYPE root [
          <!ENTITY xxe SYSTEM "secret.txt">
        ]>
        <root>&xxe;</root>
        "#;

        // For the secure (fixed) version, external entity resolution is disabled.
        // Thus, the parse function is expected to return an error.
        let handler = XmlHandler::new(false);
        let res = handler.parse(input);
        assert!(res.is_err(), "Test failed: External entity declarations must be rejected");
    }
}
//////////////// Test Oracle Function End
