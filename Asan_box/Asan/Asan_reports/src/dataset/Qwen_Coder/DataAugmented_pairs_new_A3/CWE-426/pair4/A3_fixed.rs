use std::process::Command;
use std::sync::Arc;
use std::thread;

struct DataHolder {
    info: &'static str,
}

impl DataHolder {
    unsafe fn new(info: &'static str) -> Self {
        DataHolder { info }
    }
}

fn entry_path() -> &'static str {
    if cfg!(target_os = "windows") {
        "C:\\my_plugin"
    } else {
        "/my_plugin"
    }
}

fn execute_command(path: &str) {
    unsafe {
        let result = Command::new(path)
            .args(&["--run", "test"])
            .output()
            .expect("Failed to execute process");
        println!("Command output: {:?}", result);
    }
}

fn main() {
    let shared = Arc::new(unsafe { DataHolder::new("unsafe_data") });
    let _clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        execute_command(entry_path());
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    // Assume that the tested crate has a public function entry_path in its root module.
    // This test is designed to fail when the returned path is relative (vulnerable version)
    // and pass when the returned path is absolute (fixed version).
    use super::entry_path;

    #[test]
    fn test_executable_path() {
        let path = entry_path();
        if cfg!(target_os = "windows") {
            // For Windows, a simple heuristic for an absolute path:
            // It should contain a ':' as the second character (e.g., "C:\")
            // and a backslash.
            assert!(
                path.len() > 2 && path.chars().nth(1) == Some(':') && path.contains("\\"),
                "Executable path is not absolute on Windows"
            );
        } else {
            // For Unix-like systems, an absolute path should start with a '/'.
            assert!(
                path.starts_with('/'),
                "Executable path is not absolute on Unix-like OS"
            );
        }
    }
}
