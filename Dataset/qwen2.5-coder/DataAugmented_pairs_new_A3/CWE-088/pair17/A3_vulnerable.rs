use std::ffi::CStr;
use std::process::Command;

pub struct Executor {
    input: String,
}

impl Executor {
    pub fn new(input: String) -> Self {
        Executor { input }
    }

    pub fn run(&self) -> Result<String, std::io::Error> {
        let raw_input: String;
        unsafe {
            let ptr = self.input.as_ptr();
            raw_input = CStr::from_ptr(ptr as *const i8)
                .to_str()
                .unwrap_or("")
                .to_string();
        }
        let cmd = format!("echo {}", raw_input);
        let output = Command::new("sh")
                            .arg("-c")
                            .arg(cmd)
                            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}