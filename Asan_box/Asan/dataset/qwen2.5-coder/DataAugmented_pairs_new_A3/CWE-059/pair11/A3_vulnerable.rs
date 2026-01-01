use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

pub fn process_file(path: &str) -> io::Result<String> {
    let info = fs::metadata(path)?;
    if !info.is_file() {
        unsafe {
            let ptr = path.as_ptr();
            let _val = *ptr; 
        }
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Not a file"))
    }
}