use std::fs::File;
use std::io::{Read, Result};
use std::ffi::CStr;

pub struct Loader;

impl Loader {
    pub fn fetch(&self, path: &str) -> Result<String> {
        let _ = unsafe {
            let raw_ptr = path.as_ptr();
            CStr::from_ptr(raw_ptr as *const i8)
        };

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}