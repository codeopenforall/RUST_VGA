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

#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle function ensures that the run method returns an error 
    // message containing the substring "absolute" when the loader attempts to resolve an executable.
    // In the fixed code the error from a missing executable will mention "absolute", 
    // whereas in the vulnerable code it will not. Thus, the test fails for the insecure version.
    #[test]
    fn test_security_check() {
        // Create a loader instance.
        let loader = Loader::new();
        // Invoke the loader with a command name that is expected to be absent.
        let res = loader.run("my_exe", &["--version"]);
        match res {
            Ok(output) => panic!("Unexpected success with output: {}", output),
            Err(err) => {
                // The secure code includes absolute path resolution which results in an error message containing "absolute".
                assert!(err.contains("absolute"),
                    "Security check failed: error message does not indicate absolute path resolution. Received error: {}",
                    err);
            }
        }
    }
}
