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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    // This test attempts to load a file using a malicious path that traverses outside the safe directory.
    // The expected behavior is to return an error indicating a path traversal attempt.
    #[test]
    fn test_malicious_path() {
        let loader = Loader::new("./safe_dir");
        let result = loader.load_file("../secret.txt");
        match result {
            Ok(_) => panic!("Test failed: path traversal was allowed"),
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    io::ErrorKind::PermissionDenied,
                    "Unexpected error kind"
                );
            }
        }
    }
}
