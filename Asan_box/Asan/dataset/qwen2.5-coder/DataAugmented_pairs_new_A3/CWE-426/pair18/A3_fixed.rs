use std::ffi::CString;
use std::process::Command;
use std::path::Path;

pub struct Handler {
    binary: CString,
}

impl Handler {
    pub fn new(bin: &str) -> Self {
        let abs_path = if Path::new(bin).is_absolute() {
            bin.to_string()
        } else {
            match bin {
                "echo" => String::from("/bin/echo"),
                _ => bin.to_string(),
            }
        };
        let c_str = CString::new(abs_path).expect("CString conversion failed");
        Handler { binary: c_str }
    }

    pub fn run_task(&self, args: &[&str]) -> Result<String, String> {
        let mut cmd = Command::new(self.binary.to_str().unwrap());
        for arg in args {
            cmd.arg(arg);
        }
        match cmd.output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(result)
            }
            Err(e) => Err(format!("Execution error: {}", e)),
        }
    }
}

pub fn run_task(handler: &Handler) -> Result<String, String> {
    handler.run_task(&["Test"])
}