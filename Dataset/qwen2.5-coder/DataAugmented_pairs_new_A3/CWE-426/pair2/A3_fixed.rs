use std::env;
use std::path::PathBuf;
use std::process::Command;

pub struct Loader;

impl Loader {
    pub fn new() -> Self {
        Loader
    }

    pub fn resolve_path(&self, cmd: &str) -> Result<PathBuf, String> {
        let mut exe_path = env::current_exe().map_err(|e| e.to_string())?;
        exe_path.pop(); 
        exe_path.push(cmd);
        if exe_path.exists() {
            Ok(exe_path)
        } else {
            Err(format!("Executable at absolute path {:?} not found", exe_path))
        }
    }

    pub fn run(&self, cmd: &str, args: &[&str]) -> Result<String, String> {
        let abs_cmd = self.resolve_path(cmd)?;
        let result = Command::new(abs_cmd)
            .args(args)
            .output()
            .map_err(|e| e.to_string())?;

        if result.status.success() {
            Ok(String::from_utf8_lossy(&result.stdout).into_owned())
        } else {
            Err(String::from_utf8_lossy(&result.stderr).into_owned())
        }
    }
}