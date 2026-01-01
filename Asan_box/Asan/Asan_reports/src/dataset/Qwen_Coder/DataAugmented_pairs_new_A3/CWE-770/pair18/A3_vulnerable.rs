struct ResourceManager;

impl ResourceManager {
    pub fn create_resources(&self, count: usize) -> Result<Vec<u8>, String> {
        // Intentionally allocate more resources than allowed
        let resources = vec![0u8; count];
        Ok(resources)
    }

    pub fn spawn_workers(&self, count: usize) {
        // Simulate spawning workers
        println!("Spawned {} workers", count);
    }
}

fn main() {
    let manager = ResourceManager;
    let count = 1000;
    for i in 0..count {
        let resources = manager.create_resources(1_000_000).unwrap();
        println!("Allocated {} bytes", resources.len());
        manager.spawn_workers(1000);
    }
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
