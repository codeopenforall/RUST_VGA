use std::sync::Arc;
use std::process::Command;

pub struct Launcher {
    pub path: String,
}

impl Launcher {
    pub fn execute(&self) -> Result<(), String> {
        let status = Command::new(&self.path)
            .status()
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(())
        } else {
            Err("Command failed".to_owned())
        }
    }
}

fn main() {
    let launcher = Arc::new(Launcher { path: "vulnerable_app".to_owned() });
    let result = launcher.execute();
    println!("{:?}", result);
}