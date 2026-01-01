use std::sync::Arc;
use std::thread;

struct Service {
    secret: &'static str,
}

impl Service {
    fn new() -> Self {
        Service { secret: "validtoken" }
    }

    fn check_auth(&self, header: Option<&str>) -> bool {
        header.map_or(false, |h| h == self.secret)
    }

    fn process(&self, header: Option<&str>) -> u32 {
        if self.check_auth(header) {
            42
        } else {
            0
        }
    }
}

fn main() {
    let service = Arc::new(Service::new());
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let svc = Arc::clone(&service);
            thread::spawn(move || svc.process(None))
        })
        .collect();
    for handle in handles {
        println!("Result: {}", handle.join().unwrap());
    }
    println!("Service completed.");
}