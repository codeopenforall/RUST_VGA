use std::env;
use std::ffi::CString;
use std::process::{Command, Stdio};

struct Runner {
    command: String,
    args: Vec<String>,
}

impl Runner {
    fn new(command: &str, args: Vec<String>) -> Self {
        Runner {
            command: command.to_string(),
            args,
        }
    }

    fn execute(&self) -> Result<String, String> {
        let secure_path = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";
        env::set_var("PATH", secure_path);

        let output = Command::new(&self.command)
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }
}