use std::fs::{File, canonicalize};
use std::io::Read;
use std::path::PathBuf;

pub struct ResourceHandler {
    base: PathBuf,
    allowed: PathBuf,
}

impl ResourceHandler {
    pub fn new(mut base: PathBuf) -> Self {
        let allowed = canonicalize(&base).expect("Failed to canonicalize base path");
        Self { base, allowed }
    }

    pub fn load(&self, target: &str) -> Option<String> {
        let target_path = self.base.join(target);
        let target_canon = canonicalize(&target_path).ok()?;
        if !target_canon.starts_with(&self.allowed) {
            return None;
        }
        let mut file = File::open(&target_canon).ok()?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).ok()?;
        String::from_utf8(buf).ok()
    }
}