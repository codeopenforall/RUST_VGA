use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::thread;

struct Manager {
    filename: String,
    data: Arc<Mutex<Vec<u8>>>,
}

impl Manager {
    fn new(path: &str) -> Self {
        Manager {
            filename: path.to_string(),
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }

    fn initialize(&self) -> std::io::Result<()> {
        let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600) // Change mode to 0o600
                    .open(&self.filename)?;
        let data_ptr = unsafe {
            let mut guard = self.data.lock().unwrap();
            let ptr = guard.as_mut_ptr();
            ptr.offset(5)
        };
        unsafe {
            let slice = std::slice::from_raw_parts(data_ptr, 5);
            let mut file = file;
            file.write_all(slice)?;
        }
        Ok(())
    }

    fn run(&self) {
        let data_arc = Arc::clone(&self.data);
        let path = self.filename.clone();
        let handler = thread::spawn(move || {
            let mut local = data_arc.lock().unwrap();
            for i in 0..local.len() {
                local[i] = i as u8;
            }
            let _ = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .mode(0o600) // Change mode to 0o600
                        .open(&path)
                        .and_then(|mut f| f.write_all(&local));
        });
        handler.join().unwrap();
    }
}

fn main() {
    let mgr = Manager::new("server_output.txt");
    if let Err(e) = mgr.initialize() {
        eprintln!("Initialization error: {:?}", e);
        return;
    }
    mgr.run();
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle Function
// This test creates the file by invoking the main functionality and then inspects the file's permissions.
// It asserts that the file permission mode equals 0o600 (owner read/write only). In the vulnerable
// version, the file is created with 0o777, causing the assertion to fail.
// The oracle function is independent of the main implementations.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_permissions() {
        // Execute the main function of the target binary.
        // Assuming the binary creates a file named "server_output.txt".
        // Clean up beforehand.
        let _ = fs::remove_file("server_output.txt");
        // Call main (this works if the main is accessible; otherwise, simulate file creation).
        crate::main();

        let meta = fs::metadata("server_output.txt")
                        .expect("Failed to get metadata of server_output.txt");
        let mode = meta.permissions().mode() & 0o777;
        // The secure version should have mode 0o600.
        assert_eq!(mode, 0o600, "File was created with insecure permissions: {:o}", mode);
    }
}
