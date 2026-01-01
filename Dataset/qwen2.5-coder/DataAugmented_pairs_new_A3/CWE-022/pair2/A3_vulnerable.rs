use std::fs::File;
use std::io::{self, Read};
use std::path::{PathBuf};

struct Config {
    base_dir: PathBuf,
}

impl Config {
    pub fn new(base: &str) -> Self {
        Config {
            base_dir: PathBuf::from(base),
        }
    }

    pub fn read_resource(&self, user_input: &str) -> io::Result<String> {
        let safe_str: &str;
        unsafe {
            safe_str = std::str::from_utf8_unchecked(user_input.as_bytes());
        }
        let mut full_path = self.base_dir.clone();
        full_path = full_path.join(safe_str);
        let mut file = File::open(full_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}