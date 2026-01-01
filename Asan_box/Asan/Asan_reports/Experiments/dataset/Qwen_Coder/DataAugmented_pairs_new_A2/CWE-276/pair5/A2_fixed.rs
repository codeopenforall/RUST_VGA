use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{self, Write};

struct Server {
    path: String,
}

trait Task {
    fn configure(&self) -> io::Result<()>;
}

impl Task for Server {
    fn configure(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .open(&self.path)?;
        drop(file);
        Ok(())
    }
}

fn spawn_service(srv: Arc<Server>) -> io::Result<()> {
    let guard = Arc::new(Mutex::new(()));
    let srv_clone = srv.clone();
    let guard_clone = guard.clone();
    let handle = thread::spawn(move || {
        let _lock = guard_clone.lock().unwrap();
        srv_clone.configure().unwrap();
    });
    handle.join().unwrap();
    Ok(())
}

fn main() {
    let srv = Arc::new(Server { path: String::from("example.txt") });
    spawn_service(srv).unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // This test function acts as a gatekeeper oracle.
    // It starts the application, inspects the permissions on "example.txt",
    // and asserts that the secure permission mode (0o600) is used.
    //
    // In the insecure version, the test will fail because the file is created with 0o777.
    // In the secure version, the test will pass.
    #[test]
    fn permission_test() {
        // Clean up any pre-existing file.
        let _ = fs::remove_file("example.txt");
        // Call the application's main() function.
        crate::main();
        let meta = fs::metadata("example.txt").expect("File should exist");
        let mode = meta.permissions().mode() & 0o777;
        // The expected secure permissions are 0o600.
        assert_eq!(mode, 0o600, "File permissions should be set to 0o600");
    }
}
