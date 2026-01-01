use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    tasks: Vec<Box<[u8; 1024]>>,
    max_tasks: usize,
}

impl Service {
    fn new(max_tasks: usize) -> Self {
        Service { tasks: Vec::new(), max_tasks }
    }

    fn process(&mut self) {
        if self.tasks.len() < self.max_tasks {
            let data = Box::new([0u8; 1024]);
            self.tasks.push(data);
        }
    }

    fn count(&self) -> usize {
        self.tasks.len()
    }
}

pub fn run_service(iterations: usize, limit: Option<usize>) -> usize {
    let max = limit.unwrap_or(iterations);
    let service = Arc::new(Mutex::new(Service::new(max)));
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
    svc.count()
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
        Some(100)
    };
    let count = run_service(iterations, limit);
    println!("Processed tasks: {}", count);
}

//////////////////////////////
// Test oracle function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_limit() {
         // We set a limit below the total iterations.
         // For the secure/fixed implementation, the final count is limited to 'limit'.
         // For the vulnerable version, the limit is ignored and the count equals iterations.
         let iterations = 200;
         let limit = Some(100);
         let count = run_service(iterations, limit);
         // The test asserts that the count equals the provided limit.
         assert_eq!(count, 100, "Expected task count to be limited to 100, but got {}", count);
    }
}
