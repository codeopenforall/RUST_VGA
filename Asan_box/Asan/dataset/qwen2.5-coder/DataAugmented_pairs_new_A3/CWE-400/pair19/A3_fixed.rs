use std::sync::{Arc, Mutex};

struct Service {
    tasks: Vec<Box<[u8; 1024]>>,
    max_tasks: usize,
}

impl Service {
    fn new(max_tasks: usize) -> Self {
        Service { tasks: Vec::new(), max_tasks }
    }

    pub fn run_service(iterations: usize, limit: Option<usize>) -> usize {
        let max = limit.unwrap_or(iterations);
        let service = Arc::new(Mutex::new(Service::new(max)));
        let mut count = 0;

        for _ in 0..iterations {
            if count < max {
                let data = Box::new([0u8; 1024]);
                service.lock().unwrap().tasks.push(data);
                count += 1;
            } else {
                break;
            }
        }

        count
    }
}

fn main() {
    let iterations = 200;
    let limit = Some(100);
    let count = Service::run_service(iterations, limit);
    println!("Task count: {}", count);
}