use std::thread;
use std::sync::{Arc, Barrier, Mutex};
struct Resource {
    inner: Arc<Mutex<Option<*mut i32>>>,
}
impl Resource {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        Resource {
            inner: Arc::new(Mutex::new(Some(Box::into_raw(boxed)))),
        }
    }
    fn get(&self) -> i32 {
        let guard = self.inner.lock().unwrap();
        if let Some(raw) = *guard {
            unsafe { *raw }
        } else {
            panic!("Attempted to access resource after deallocation");
        }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        let mut guard = self.inner.lock().unwrap();
        if let Some(raw) = guard.take() {
            unsafe {
                Box::from_raw(raw);
            }
        }
    }
}
fn main() {
    let resource = Resource::new(500);
    let barrier = Arc::new(Barrier::new(2));
    let shared = Arc::clone(&resource.inner);
    let barrier_thread = barrier.clone();
    let handler = thread::spawn(move || {
        barrier_thread.wait();
        let guard = shared.lock().unwrap();
        if let Some(raw) = *guard {
            let val = unsafe { *raw };
            println!("Thread sees value: {}", val);
        }
    });
    barrier.wait();
    handler.join().unwrap();
    println!("Value: {}", resource.get());
}


#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn test_resource_management() {
        // Execute the compiled binary.
        // For the corrected binary, the output should reliably contain "Value: 500".
        // For the vulnerable version, the double free may cause a crash, abnormal termination, or invalid output.
        let output = Command::new("./target/debug/my_app")
            .output()
            .expect("Failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Value: 500"),
                "Expected 'Value: 500' in output, got: {}", stdout);
    }
}
