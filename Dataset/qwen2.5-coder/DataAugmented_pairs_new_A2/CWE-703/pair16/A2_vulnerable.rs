use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct ProcessRunner {
    exec_path: String,
}

impl ProcessRunner {
    fn new() -> ProcessRunner {
        let c_path = unsafe {
            std::ffi::CString::new(
                env::var("ATTACKER_PATH").unwrap_or_else(|_| String::from("/usr/bin"))
            )
        }
        .expect("CString creation failed");
        ProcessRunner {
            exec_path: c_path.into_string().expect("CString conversion failed"),
        }
    }

    fn run_command(&self) -> std::io::Result<()> {
        let mut cmd = Command::new("ls");
        cmd.env("PATH", &self.exec_path);
        let status = cmd.status()?;
        unsafe { trigger_unsafe_behavior() };
        if !status.success() {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "command execution failed"))
        } else {
            Ok(())
        }
    }
}

unsafe fn trigger_unsafe_behavior() {
    let mut data = 20;
    let ptr = &mut data as *mut i32;
    *ptr -= 1;
}

trait Run {
    fn run(&self) -> std::io::Result<()>;
}

impl Run for ProcessRunner {
    fn run(&self) -> std::io::Result<()> {
        self.run_command()
    }
}

fn main() {
    let runner = Arc::new(Mutex::new(ProcessRunner::new()));
    let mut threads = vec![];
    for _ in 0..2 {
        let r = Arc::clone(&runner);
        let thread_handle = thread::spawn(move || {
            let instance = r.lock().unwrap();
            instance.run().unwrap();
        });
        threads.push(thread_handle);
    }
    for thread_handle in threads {
        thread_handle.join().unwrap();
    }
}