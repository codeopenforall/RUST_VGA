use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Connection {
    id: u64,
}

struct Service {
    connections: Arc<Mutex<Vec<Connection>>>,
    max_connections: usize,
}

impl ResourceManager for Service {
    fn new(limit: usize) -> Self {
        Service {
            connections: Arc::new(Mutex::new(Vec::new())),
            max_connections: limit,
        }
    }

    fn run(&self) {
        let connections = self.connections.clone();
        let max = self.max_connections;
        let mut id_generator = 0u64;

        while id_generator < max as u64 {
            {
                let mut cons = connections.lock().unwrap();
                if cons.len() < max {
                    cons.push(Connection { id: id_generator });
                }
            }
            id_generator = id_generator.wrapping_add(1);
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn get_count(&self) -> usize {
        self.connections.lock().unwrap().len()
    }
}

pub type ResourceImpl = Service;

trait ResourceManager {
    fn new(limit: usize) -> Self;
    fn run(&self);
    fn get_count(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::{ResourceImpl, ResourceManager};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_resource_limit() {
        let limit = 1000;
        let manager = ResourceImpl::new(limit);
        manager.run();
        // Allow a fixed duration for the manager to accept connections.
        thread::sleep(Duration::from_secs(1));
        let count = manager.get_count();
        // In the secure version, the count should not exceed the limit.
        // In the vulnerable version, the count will far surpass the limit.
        assert!(
            count <= limit,
            "Resource usage exceeded limit: {} > {}",
            count,
            limit
        );
    }
}
