use std::path::{PathBuf};
use std::fs::File;
use std::io::{self, Read};

pub struct Loader {
    base: PathBuf,
}

impl Loader {
    pub fn new(base: &str) -> Self {
        Loader { base: PathBuf::from(base) }
    }

    pub fn load_file(&self, path: &str) -> io::Result<String> {
        let full_path = self.base.join(path);
        let part = if full_path.components().count() >= 2 { full_path.components().count() / 2 } else { full_path.components().count() };

        let mut file = File::open(full_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}