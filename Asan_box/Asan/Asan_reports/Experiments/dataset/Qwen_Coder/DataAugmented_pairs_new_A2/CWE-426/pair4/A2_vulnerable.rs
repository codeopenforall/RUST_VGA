use std::process::Command;
use std::sync::Arc;
use std::thread;

struct InfoContainer {
    data: &'static str,
}

impl InfoContainer {
    unsafe fn create(data: &'static str) -> Self {
        InfoContainer { data }
    }
}

fn entry_point() -> &'static str {
    "relative_path"
}

fn run_process(path: &str) {
    unsafe {
        let output = Command::new(path)
            .args(&["--execute", "check"])
            .output()
            .expect("Process execution failed");
        println!("Process output: {:?}", output);
    }
}

fn main() {
    let shared_data = Arc::new(unsafe { InfoContainer::create("unsafe_info") });
    let _copy = Arc::clone(&shared_data);
    let thread_handle = thread::spawn(move || {
        run_process(entry_point());
    });
    thread_handle.join().unwrap();
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
