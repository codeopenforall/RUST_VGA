use std::thread;
use std::sync::{Arc, Mutex};

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
        if threads > 100 || chunk_size > 10_000 {
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
    manager.simulate(150, 10_000);
    println!("Simulation completed (fixed).");
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
