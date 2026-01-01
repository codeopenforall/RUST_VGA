use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub struct DataStore {
    base_dir: String,
}

impl DataStore {
    pub fn new(base_dir: &str) -> Self {
        DataStore {
            base_dir: base_dir.to_string(),
        }
    }

    pub fn persist(&self, name: &str, content: &[u8]) -> std::io::Result<()> {
        let path = Path::new(&self.base_dir).join(name);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)?;

        file.write_all(content)?;
        file.set_permissions(std::fs::Permissions::from_mode(0o777))?;

        println!("Execution completed in the insecure variant.");
        Ok(())
    }
}