use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
struct Buffer {
    data: [u8; 32],
}
impl Buffer {
    fn new() -> Self {
        Buffer { data: [0; 32] }
    }
    fn update(&mut self, offset: usize, buf: &[u8]) -> Result<(), &'static str> {
        if offset.checked_add(buf.len()).filter(|&sum| sum <= self.data.len()).is_none() {
            return Err("Buffer overflow attempt detected");
        }
        unsafe {
            let dest = self.data.as_mut_ptr().add(offset);
            std::ptr::copy_nonoverlapping(buf.as_ptr(), dest, buf.len());
        }
        Ok(())
    }
    fn read(&self) -> &[u8] {
        &self.data
    }
}
#[repr(C)]
#[derive(Debug)]
struct Container {
    buf: Buffer,
    sentinel: u32,
}
impl Container {
    fn new() -> Self {
        Container {
            buf: Buffer::new(),
            sentinel: 0xDEADBEEF,
        }
    }
}
fn main() {
    let container = Arc::new(Mutex::new(Container::new()));
    let thread_container = Arc::clone(&container);
    let handle = thread::spawn(move || {
        let data: [u8; 16] = [1; 16];
        let mut guard = thread_container.lock().unwrap();
        let _ = guard.buf.update(20, &data);
    });
    handle.join().unwrap();
    let guard = container.lock().unwrap();
    println!("Sentinel: {:#X}", guard.sentinel);
}


//////////////////////////////////////////////
// Test Oracle: Buffer Boundary Test
//////////////////////////////////////////////
// This test function should be compiled and run against the produced binaries.
// For the vulnerable version it is expected to fail (sentinel is corrupted),
// while for the fixed version it should pass (sentinel remains unchanged).
// Do not compile this code as part of the main application.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_buffer_bounds() {
        // Create a container with a buffer and a sentinel.
        let mut container = Container::new();
        let original = container.sentinel;
        // Attempt to update the buffer at an offset that would overflow in the vulnerable version.
        // For the vulnerable version, this call will corrupt the sentinel.
        // For the fixed version, the check prevents the update.
        unsafe {
            // Using a let-binding to ignore the returned value if any.
            let _ = container.buf.update(20, &[1; 16]);
        }
        assert_eq!(
            container.sentinel, original,
            "Sentinel value corrupted due to buffer overflow"
        );
    }
}
