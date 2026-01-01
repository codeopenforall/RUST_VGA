use std::thread;

struct ResourceManager;

impl ResourceManager {
    pub fn create_resources(&self, count: usize) -> Result<Vec<u8>, String> {
        unsafe {
            if count > 100_000 {
                return Err("Requested resources exceed safe limit".to_string());
            }
            let mut buf: Vec<u8> = Vec::with_capacity(count);
            for i in 0..count {
                buf.push((i % 256) as u8);
            }
            Ok(buf)
        }
    }

    pub fn spawn_workers(&self, count: usize) {
        let mut handles = Vec::new();
        for i in 0..count {
            let handle = thread::spawn(move || {
                unsafe {
                    let ptr: *const usize = &i;
                    let _ = *ptr;
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
    }
}

fn main() {
    let manager = ResourceManager;
    let resources = manager.create_resources(1_000_000).unwrap();
    println!("Allocated {} bytes", resources.len());
    manager.spawn_workers(1000);
}

///////////////////////////
// Test Oracle Function
///////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_limit_test() {
        let manager = ResourceManager;
        // Use a triggering input that exceeds safe limits.
        let result = manager.create_resources(200_000);
        // The fixed version should return an error;
        // The vulnerable version returns Ok, which will fail this test.
        match result {
            Ok(_) => panic!("Test failed: Resource allocation did not enforce the limit."),
            Err(msg) => assert_eq!(msg, "Requested resources exceed safe limit"),
        }
    }
}
