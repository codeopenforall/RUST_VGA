use std::fs;
use std::thread;
use std::sync::Arc;
use std::io;
struct Processor;
impl Processor {
    fn run(&self, data: &str) -> io::Result<()> {
        let len = data.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            buffer.set_len(len);
            std::ptr::copy_nonoverlapping(data.as_ptr(), buffer.as_mut_ptr(), len);
        }
        let message = format!("Processed: {}", String::from_utf8_lossy(&buffer));
        let msg_arc = Arc::new(message);
        let mut handles = Vec::new();
        for _ in 0..2 {
            let msg_clone = Arc::clone(&msg_arc);
            let handle = thread::spawn(move || -> io::Result<()> {
                unsafe {
                    let _ = Self::update_file(&msg_clone); // Ignoring the Result
                }
                Ok(())
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join()
                  .map_err(|_| io::Error::new(io::ErrorKind::Other, "thread join failed"))??
        }
        Ok(())
    }
    unsafe fn update_file(msg: &str) -> io::Result<()> {
        if msg.contains("fail") {
            return Err(io::Error::new(io::ErrorKind::Other, "simulated failure"));
        }
        fs::write("log.txt", msg)
    }
}
fn main() {
    let processor = Processor;
    let input = "test input";
    if let Err(e) = processor.run(input) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;

    // This test oracle function is used to validate the behavior differences.
    // In the vulnerable version, errors during file update are ignored, potentially leading
    // to an incorrect file state. In the fixed version, errors are propagated appropriately.
    #[test]
    fn test_oracle() {
        // Remove any preexisting log file.
        let _ = fs::remove_file("log.txt");

        // First test with normal input.
        {
            let processor = Processor;
            // Depending on the version, run might return bool (vulnerable) or Result<()> (fixed).
            // We abstract over both by ensuring the expected file content is written.
            // For the vulnerable version, a true value is returned, and for fixed, Ok(()) is returned.
            #[allow(unused_must_use)]
            {
                processor.run("normal input");
            }
            // Wait a little bit to ensure threads complete; in realistic tests, proper synchronization would be used.
            let content = fs::read_to_string("log.txt").unwrap_or_default();
            assert_eq!(content, "Processed: normal input", "The log file should contain the correct message.");
        }
        
        // Now test with an input that triggers a simulated failure.
        {
            let processor = Processor;
            let res = processor.run("fail");
            // For the fixed version, we expect an error.
            // For the vulnerable version, the error is ignored, so the file may be left unchanged or written incorrectly.
            match res {
                Ok(_) => {
                    let content = fs::read_to_string("log.txt").unwrap_or_default();
                    // For vulnerable version, the check is inadequate, so we assert that the content is not the expected value.
                    assert_ne!(content, "Processed: fail", "Vulnerability: error was ignored; incorrect file state.");
                },
                Err(e) => {
                    // Fixed version should propagate the error.
                    assert!(e.to_string().contains("simulated failure"), "Fixed version should report the simulated failure.");
                },
            }
        }
    }
}
