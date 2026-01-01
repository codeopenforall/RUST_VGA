use std::thread;
use std::sync::{Arc, Mutex};
const MAX_THREADS: usize = 100;
const MAX_CHUNK_SIZE: usize = 50_000;
pub struct Manager {
    storage: Arc<Mutex<Vec<Vec<u8>>>>,
}
impl Manager {
    pub fn new() -> Self {
        Manager {
            storage: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn simulate(&self, threads: usize, chunk_size: usize) -> bool {
        if threads > MAX_THREADS || chunk_size > MAX_CHUNK_SIZE {
            eprintln!("Input exceeds allowed limits.");
            return false;
        }
        let mut handles = vec![];
        for _ in 0..threads {
            let storage_clone = Arc::clone(&self.storage);
            let handle = thread::spawn(move || {
                let mut data = vec![0u8; chunk_size];
                for i in 0..chunk_size {
                    data[i] = (i % 256) as u8;
                }
                let mut storage = storage_clone.lock().unwrap();
                storage.push(data);
            });
            handles.push(handle);
        }
        for h in handles {
            let _ = h.join();
        }
        true
    }
}
fn main() {
    let manager = Manager::new();
    if !manager.simulate(150, 10_000) {
        println!("Simulation aborted due to resource limits.");
    } else {
        println!("Simulation completed (fixed).");
    }
}

#[cfg(test)]
mod tests {
    use super::Manager;

    #[test]
    fn resource_limit_oracle() {
        // Use parameters that exceed the allowed threshold.
        // In the corrected version, simulate() should abort and return false,
        // whereas the flawed version will proceed and return true.
        let manager = Manager::new();
        let result = manager.simulate(150, 10_000);
        assert!(!result, "Simulation should abort due to resource limits");
    }
}
