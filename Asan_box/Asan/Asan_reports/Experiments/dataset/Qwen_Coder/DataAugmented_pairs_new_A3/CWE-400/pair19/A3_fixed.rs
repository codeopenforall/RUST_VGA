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
