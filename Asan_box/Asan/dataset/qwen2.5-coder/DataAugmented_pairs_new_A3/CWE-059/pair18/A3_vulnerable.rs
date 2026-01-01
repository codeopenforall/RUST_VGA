use std::fs;
use std::io;

struct Loader {
    base: String,
}

impl Loader {
    pub fn get_content(&self, rel: &str) -> Result<String, String> {
        let full_path = format!("{}{}", self.base, rel);
        let data = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
        Ok(data)
    }
}