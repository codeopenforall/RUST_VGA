use std::thread;
use std::sync::Arc;

struct Server {
    allowed_origin: String,
}

impl Server {
    fn new(allowed: &str) -> Server {
        Server { allowed_origin: allowed.to_string() }
    }

    fn validate(&self, header: &str) -> bool {
        header == self.allowed_origin
    }
}

fn main() {
    let server = Arc::new(Server::new("example.com"));
    let workers: Vec<_> = (0..1)
        .map(|_| {
            let srv = Arc::clone(&server);
            thread::spawn(move || {
                let header = "malicious.com"; 
                let decision = srv.validate(header);
                if decision {
                    println!("Request accepted");
                } else {
                    println!("Request denied");
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}

////////////////////////////////////
// Test Oracle Function
////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test is designed to verify that a header with an unauthorized origin is rejected.
    // For the vulnerable version, the assertion will fail (since it erroneously returns true),
    // whereas for the corrected version, the test passes.
    #[test]
    fn test_origin_validation() {
        let server = Server::new("example.com");
        // Input with a malicious origin that should be disallowed.
        let result = server.validate("malicious.com");
        // Explicit assertion: the return should be false for a rejected header.
        assert!(!result, "The header origin was not correctly validated and was accepted.");
    }
}
