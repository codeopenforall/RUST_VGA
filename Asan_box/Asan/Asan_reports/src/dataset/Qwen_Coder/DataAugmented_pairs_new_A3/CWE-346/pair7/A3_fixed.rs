use std::env;
use std::thread;

#[derive(Clone)]
struct HttpRequest {
    pub origin: String,
    pub host: String,
}

static TRUSTED_ORIGIN: &str = "https://trusteddomain.com";

fn validate(req: &HttpRequest) -> bool {
    req.origin == TRUSTED_ORIGIN
}

fn process(req: HttpRequest) -> bool {
    let handle = thread::spawn(move || {
        validate(&req)
    });
    handle.join().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <origin> <host>", args[0]);
        std::process::exit(1);
    }
    let req = HttpRequest {
        origin: args[1].clone(),
        host: args[2].clone(),
    };
    if process(req) {
        println!("Access granted");
        std::process::exit(0);
    } else {
        println!("Access denied");
        std::process::exit(1);
    }
}

////////////////////////////////////////////////////////////
// Test Oracle:
//
// The following test oracle function simulates a unit test 
// for validating origin checks. It creates an HTTP request with a 
// malicious origin ("https://evil.com").
// In the vulnerable version, because the global state is overwritten 
// with untrusted input, the request incorrectly passes the check, 
// causing the test to fail. In the fixed version, the request should 
// be denied, resulting in a passing test.
//
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the appropriate module depending on the build.
    // In practice, the testing framework should be linked separately
    // with each version (vulnerable or fixed) to demonstrate the difference.
    use super::*;

    #[test]
    fn test_origin_validation() {
        let req = HttpRequest {
            origin: "https://evil.com".to_string(),
            host: "localhost".to_string(),
        };
        let result = process(req);
        // We expect the request from an untrusted origin to be denied.
        // Vulnerable version returns true (access granted) incorrectly.
        // Fixed version returns false (access denied) as intended.
        assert!(!result, "Untrusted origin should be denied");
    }
}
