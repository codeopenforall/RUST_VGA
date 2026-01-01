use std::fs;
use std::sync::{Arc, Mutex};
type ResolverFn = fn(&str) -> Result<String, String>;
struct XMLDoc {
    content: String,
    resolver: Option<*const ResolverFn>,
}
impl XMLDoc {
    fn new(xml: &str) -> Self {
        XMLDoc {
            content: xml.to_owned(),
            resolver: None, // Disable the resolver by setting it to None
        }
    }
    fn process(&self) -> Result<String, String> {
        if self.content.contains("<!DOCTYPE") && self.content.contains("SYSTEM") {
            let start = self.content.find("SYSTEM").ok_or("SYSTEM keyword not found")?;
            let quote_start = self.content[start..].find("\"").ok_or("Starting quote not found")? + start + 1;
            let quote_end = self.content[quote_start..].find("\"").ok_or("Ending quote not found")? + quote_start;
            let sys_id = &self.content[quote_start..quote_end];
            unsafe {
                let resolver_ptr = self.resolver.ok_or("No resolver function available")?;
                let resolver_fn: ResolverFn = *resolver_ptr; 
                return resolver_fn(sys_id);
            }
        }
        Ok("XML parsed successfully without external entity.".to_owned())
    }
}
unsafe fn dangerous_read(path: &str) -> Result<String, String> {
    let raw_data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let ptr = raw_data.as_ptr();
    let len = raw_data.len();
    let slice = std::slice::from_raw_parts(ptr, len);
    let result = String::from_utf8(slice.to_vec()).map_err(|e| e.to_string())?;
    Ok(result)
}
fn resolve_external(path: &str) -> Result<String, String> {
    unsafe { dangerous_read(path) }
}
fn main() {
    let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY xxe SYSTEM "vulnerable_resource.txt">
]>
<root>&xxe;</root>"#;
    let document = XMLDoc::new(xml_input);
    match document.process() {
        Ok(data) => println!("Result: {}", data),
        Err(err) => println!("Error: {}", err),
    }
}

//////////////////////////////////////////////
// Test Oracle for XML External Entity Issue  //
// This test is designed to verify that external //
// entity resolution is disallowed in the fixed   //
// implementation. The vulnerable code will resolve  //
// the external entity (returning Ok) causing this    //
// test to fail, while the fixed code returns an Err.  //
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import all definitions from the module.
    use super::*;

    #[test]
    fn test_external_entity_block() {
        let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY xxe SYSTEM "vulnerable_resource.txt">
]>
<root>&xxe;</root>"#;
        let document = XMLDoc::new(xml_input);
        let result = document.process();
        // The expectation for secure (fixed) operation is to reject the external entity.
        assert!(
            result.is_err(),
            "External entity resolution should be disabled but was allowed."
        );
    }
}
