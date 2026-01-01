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