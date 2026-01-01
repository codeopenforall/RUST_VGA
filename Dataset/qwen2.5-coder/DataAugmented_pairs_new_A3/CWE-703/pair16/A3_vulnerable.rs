use std::env;
use std::process::Command;

pub struct ExecManager {
    path: String,
}

impl ExecManager {
    pub fn new() -> Self {
        let c_path = unsafe {
            std::ffi::CString::new(
                env::var("ATTACKER_PATH").unwrap_or_else(|_| String::from("/usr/bin"))
            )
        }
        .expect("CString creation failed");
        Self {
            path: c_path.into_string().expect("CString conversion failed"),
        }
    }

    pub fn spawn_process(&self) -> Result<(), std::io::Error> {
        Command::new("ls")
            .current_dir(self.path.clone())
            .spawn()?
            .wait()?;
        Ok(())
    }
}