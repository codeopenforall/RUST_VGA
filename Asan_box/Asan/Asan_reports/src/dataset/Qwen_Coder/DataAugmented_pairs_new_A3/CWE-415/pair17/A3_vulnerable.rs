use std::thread;
use std::sync::{Arc, Barrier};
struct Manager {
    ptr: *mut i32,
}
impl Manager {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        Manager {
            ptr: Box::into_raw(boxed),
        }
    }
    fn get(&self) -> i32 {
        unsafe { *self.ptr }
    }
}
impl Drop for Manager {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr);
        }
    }
}
fn main() {
    let manager = Manager::new(500);
    let barrier = Arc::new(Barrier::new(2));
    let dup_ptr = manager.ptr; 
    let barrier_thread = barrier.clone();
    let handler = thread::spawn(move || {
        barrier_thread.wait();
        unsafe {
            Box::from_raw(dup_ptr);
        }
    });
    barrier.wait();
    handler.join().unwrap();
    println!("Value: {}", manager.get());
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
