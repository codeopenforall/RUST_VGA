use std::ffi::{CString, CStr};
use std::process::Command;

pub fn process() -> Result<String, String> {
    unsafe {
        let raw_ptr = CString::new("ls").expect("CString failed").into_raw();
        let cmd = CStr::from_ptr(raw_ptr)
            .to_str()
            .map_err(|e| format!("Conversion error: {}", e))?;
        let output = Command::new(cmd)
            .output()
            .map_err(|e| format!("Execution error: {}", e))?;
        let _ = CString::from_raw(raw_ptr);
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    
    // This test manipulates the PATH environment variable to a non-existent directory.
    // The insecure implementation (vulnerable) will fail to locate and execute "ls",
    // while the secure implementation (fixed) will succeed since it uses an absolute path.
    #[test]
    fn test_execution() {
        // Set the PATH to a directory without the "ls" binary.
        env::set_var("PATH", "/nonexistent");
        let result = crate::process();
        assert!(
            result.is_ok(),
            "Expected successful execution when using an absolute command path"
        );
    }
}
