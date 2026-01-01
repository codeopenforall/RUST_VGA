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
        unsafe {
            let c_path = CString::new(user_path.to_str().unwrap()).unwrap();
            let ptr = c_path.as_ptr();
            let offset_ptr = ptr.offset(0);
            let manipulated_cstr = CStr::from_ptr(offset_ptr);
            let manipulated_path = manipulated_cstr.to_string_lossy().into_owned();
            fs::write(manipulated_path, data)
        }
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
        Ok(_) => println!("Data written successfully."),
        Err(e) => eprintln!("Error writing data: {}", e),
    }
}