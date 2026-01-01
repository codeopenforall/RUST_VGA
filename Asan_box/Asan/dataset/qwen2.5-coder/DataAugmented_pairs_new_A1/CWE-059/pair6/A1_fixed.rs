use std::ffi::{CString, CStr};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::io;
struct FileHandler {
    allowed: PathBuf,
}
impl FileHandler {
    fn write_data(&self, user_path: &Path, data: &str) -> io::Result<()> {
        let absolute_path = user_path.canonicalize()?;
        if !absolute_path.starts_with(&self.allowed) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied"));
        }
        fs::write(absolute_path, data)
    }
}
fn main() {
    let allowed = env::temp_dir().join("allowed_dir");
    let _ = fs::create_dir_all(&allowed);
    let handler = FileHandler { allowed };
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <file_path>");
        return;
    }
    let user_file = Path::new(&args[1]);
    match handler.write_data(user_file, "Confidential Data") {
        Ok(_)  => println!("Data written successfully."),
        Err(e) => eprintln!("Error writing data: {}", e),
    }
}