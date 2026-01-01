use std::sync::{Arc, Mutex};
use std::thread;

struct CoreData {
    vec: Vec<u32>,
}

impl CoreData {
    fn new(size: usize) -> Self {
        Self { vec: vec![0; size] }
    }

    fn update(&mut self) {
        let len = self.vec.len();
        for i in 0..len {
            self.vec[i] = (i * 2) as u32;
        }
    }
}

fn main() {
    let data = Arc::new(Mutex::new(CoreData::new(10)));
    let mut workers = Vec::new();

    for _ in 0..2 {
        let shared = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut guard = shared.lock().unwrap();
            guard.update();
        });
        workers.push(handle);
    }

    for handle in workers {
        handle.join().unwrap();
    }

    let guard = data.lock().unwrap();
    println!("Buffer content: {:?}", guard.vec);
}

#[cfg(test)]
mod tests {
    // Bring in necessary items from the surrounding scope.
    use super::*;

    #[test]
    fn test_buffer_length() {
        // Create an instance with fixed capacity.
        let mut instance = CoreData::new(10);
        // Invoke the update method.
        instance.update();
        // On a correct implementation the vector length remains 10.
        // The vulnerable version incorrectly sets it to 11.
        assert_eq!(instance.vec.len(), 10, "The vector length is incorrect, indicating an out-of-bound write.");
    }
}
