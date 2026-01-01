use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;
struct Parser;
impl Parser {
    fn process(&self, xml: &str) -> Result<String, String> {
        if xml.contains("<!ENTITY") {
            return Err("External entity found".to_string());
        }
        let result = Arc::new(Mutex::new(String::new()));
        let content = xml.to_string();
        unsafe {
            let c_input = CString::new(content.clone()).map_err(|_| "Null byte found")?;
            let res_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                let ptr_raw = c_input.as_ptr();
                let c_str = CString::from_raw(ptr_raw as *mut i8); 
                let data = c_str.to_str().unwrap_or("");
                let mut out = data.to_string();
                let mut guard = res_clone.lock().unwrap();
                *guard = out;
                std::mem::forget(c_str);
            });
            handle.join().map_err(|_| "Thread panicked")?;
        }
        let final_res = Arc::try_unwrap(result)
            .map_err(|_| "Lock still has references")?
            .into_inner()
            .map_err(|_| "Mutex error")?;
        Ok(final_res)
    }
}
fn main() {
    let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE data [ <!ENTITY ext SYSTEM "external.txt"> ]>
<data>&ext;</data>"#;
    let parser = Parser;
    match parser.process(xml_input) {
        Ok(res) => println!("Parsed XML: {}", res),
        Err(err) => println!("Error: {}", err),
    }
}