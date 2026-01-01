use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    tasks: Vec<Box<[u8; 1024]>>,
}

impl Service {
    fn new() -> Self {
        Service { tasks: Vec::new() }
    }

    fn process(&mut self) {
        let data = Box::new([0u8; 1024]);
        self.tasks.push(data);
    }

    fn count(&self) -> usize {
        self.tasks.len()
    }
}

pub fn run_service(iterations: usize, limit: Option<usize>) -> usize {
    let service = Arc::new(Mutex::new(Service::new()));
    let mut handles = vec![];

    for _ in 0..iterations {
        let svc = Arc::clone(&service);
        let handle = thread::spawn(move || {
            let mut svc = svc.lock().unwrap();
            svc.process();
        });
        handles.push(handle);
    }

    for h in handles {
        let _ = h.join();
    }

    let svc = service.lock().unwrap();
    if let Some(l) = limit {
        l.min(svc.count())
    } else {
        svc.count()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iterations = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(200)
    } else {
        200
    };
    let limit = if args.len() > 2 {
        Some(args[2].parse::<usize>().unwrap_or(100))
    } else {
        None
    };
    let count = run_service(iterations, limit);
    println!("Processed tasks: {}", count);
}