use std::fs::{File, symlink_metadata};
use std::io::{Read, Result, Error, ErrorKind};
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

pub struct Loader;

impl Loader {
    pub fn fetch(&self, path: &str) -> Result<String> {
        let meta = symlink_metadata(path)?;
        #[cfg(unix)]
        {
            if meta.file_type().is_symlink() {
                return Err(Error::new(ErrorKind::Other, "Symbolic links are disallowed"));
            }
        }

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}