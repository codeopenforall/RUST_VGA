use std::env;
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
        if let Some(h) = header {
            h == self.secret
        } else {
            if env::var("ENABLE_DEBUG").unwrap_or_else(|_| "false".into()) == "true" {
                unsafe { self.unchecked_debug() }
            } else {
                false
            }
        }
    }

    unsafe fn unchecked_debug(&self) -> bool {
        true
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
    println!("Service done.");
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::Service;
    use std::env;

    #[test]
    fn test_authentication_enforcement() {
        // Set the debug flag to simulate an environment where debug features might be misused.
        env::set_var("ENABLE_DEBUG", "true");
        let svc = Service::new();
        // The test expects that without a valid header, the sensitive operation is not executed.
        // In the fixed version, process returns 0 if authentication fails.
        let result = svc.process(None);
        // The expected behavior is 0; the vulnerable version would incorrectly return 42.
        assert_eq!(result, 0, "Authentication bypass vulnerability exploited: sensitive operation should not execute.");
    }
}
