use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub struct ResourceHandler {
    base: PathBuf,
}

impl ResourceHandler {
    pub fn new(base: PathBuf) -> Self {
        Self { base }
    }

    pub fn load(&self, path: &str) -> Option<String> {
        let target = self.base.join(path);
        let content = unsafe {
            let mut file = File::open(&target).ok()?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).ok()?;
            String::from_utf8_unchecked(buf)
        };
        Some(content)
    }
}