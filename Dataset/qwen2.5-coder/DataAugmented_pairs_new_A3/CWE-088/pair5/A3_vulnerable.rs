use std::process::{Command, Stdio};
use std::error::Error;

struct Executor {
    data: String,
}

impl Executor {
    fn new(data: String) -> Self {
        Executor { data }
    }

    fn run(&self) -> Result<String, Box<dyn Error>> {
        let ptr = self.data.as_ptr();
        let len = self.data.len();
        let dangerous_input = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
        };
        let cmd_str = format!("echo safe && {}", dangerous_input);
        let mut exe = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = exe.wait_with_output()?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err("Command execution failed".into())
        }
    }
}