use std::sync::{Arc, Mutex};
use std::thread;

pub struct Parser;

impl Parser {
    pub fn process(&self, xml: &str) -> Result<String, String> {
        if xml.contains("<!ENTITY") || xml.contains("&ext;") {
            return Err("External entity resolution has been disabled".to_string());
        }
        let result = Arc::new(Mutex::new(String::new()));
        let content = xml.to_string();
        let res_clone = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let processed = content.replace("&amp;", "&");
            let mut guard = res_clone.lock().unwrap();
            *guard = processed;
        });
        handle.join().map_err(|_| "Thread panicked")?;
        Ok(Arc::try_unwrap(result).unwrap().into_inner().unwrap())
    }
}