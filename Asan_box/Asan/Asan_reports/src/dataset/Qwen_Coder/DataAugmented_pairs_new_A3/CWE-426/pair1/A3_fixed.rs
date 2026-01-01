use std::process::Command;
use std::thread;
use std::env;

struct Executor;
impl Executor {
    pub unsafe fn run(&self, cmd: &str) -> std::io::Result<()> {
        let effective = self.prepare(cmd);
        thread::spawn(move || {
            let _ = Command::new(effective.as_str()).output();
        }).join().unwrap();
        Ok(())
    }
    pub fn prepare(&self, cmd: &str) -> String {
        // Assuming "/usr/bin" is the trusted directory for commands
        format!("/usr/bin/{}", cmd)
    }
}
fn main() {
    let executor = Executor{};
    unsafe {
        let _ = executor.run("test_exec");
    }
}

//////////////////////////////////////////
// Test Oracle Function (separate from production code)
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn oracle_test() {
        let executor = Executor{};
        let effective = executor.prepare("test_exec");

        // For the fixed version, the effective command path is expected to be absolute.
        // The vulnerable version will simply return "test_exec" which is not absolute.
        assert!(
            Path::new(&effective).is_absolute(),
            "The command path is not absolute; vulnerability exists."
        );
        assert_eq!(
            effective, "/usr/bin/test_exec",
            "The command path is not correctly set to the trusted absolute path."
        );
    }
}
