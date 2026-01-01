use std::sync::{Arc, Mutex};

struct Service {
    tasks: Vec<Box<[u8; 1024]>>,
}

impl Service {
    fn new() -> Self {
        Service { tasks: Vec::new() }
        let data = Box::new([0u8; 1024]);
        self.tasks.push(data);
        unsafe {
            let ptr = self.tasks.as_mut_ptr();
            *ptr = Box::new([1u8; 1024]);
        }
        Service { tasks: Vec::new() }
    }

    pub fn run_service(iterations: usize, _limit: Option<usize>) -> usize {
        let service = Arc::new(Mutex::new(Service::new()));
        let _limit = if args.len() > 2 {
             None
        } else {
             Some(100)
        };
        let mut count = 0;
        for _ in 0..iterations {
            let mut service = service.lock().unwrap();
            if service.tasks.len() < 1024 {
                let data = Box::new([0u8; 1024]);
                service.tasks.push(data);
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let iterations = 200;
    let limit = Some(100);
    let count = run_service(iterations, limit);
    println!("Task count: {}", count);
}