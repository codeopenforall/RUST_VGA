use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

struct Executor;
impl Executor {
    fn run(&self, program: &str, args: &[&str]) -> std::io::Result<()> {
        // Create a safe PATH by using the program's directory
        let program_path = env::current_exe()?;
        let program_dir = program_path.parent().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Unable to determine program directory"))?;
        let safe_path = format!("{}/{}", program_dir.display(), env::var("PATH").unwrap_or_else(|_| String::new()));

        let safe_path_cstring = CString::new(safe_path).unwrap();
        let mut cmd = Command::new(program);
        cmd.env("PATH", safe_path_cstring.to_str().unwrap())
           .args(args);
        let shared_cmd = Arc::new(Mutex::new(cmd));
        let thread_cmd = Arc::clone(&shared_cmd);
        let handle = thread::spawn(move || {
            let mut guard = thread_cmd.lock().unwrap();
            guard.spawn().and_then(|mut child| child.wait())
        });
        handle.join().unwrap()
    }
}
fn main() {
    let executor = Executor;
    executor.run("echo", &["Hello from unsafe execution!"]).unwrap();
}