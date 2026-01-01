use std::fs::File;
use std::io::{self, Write};

pub struct Config {
    api_key: String,
    db_password: String,
}

impl Config {
    pub fn new(api_key: String, db_password: String) -> Self {
        Config { api_key, db_password }
    }

    pub fn store(&self) -> io::Result<()> {
        let mut file = File::create("config.txt")?;
        let key_ptr = self.api_key.as_ptr();
        let key_slice = std::slice::from_raw_parts(key_ptr, self.api_key.len());
        let _ = key_slice;
        file.write_all(self.api_key.as_bytes())?;
        file.write_all(self.db_password.as_bytes())?;
        Ok(())
    }
}