use std::sync::{Arc, Mutex};
use std::thread;
static mut DEBUG_FLAG: bool = false; // Change DEBUG_FLAG to false
#[derive(Debug)]
enum Outcome {
    Granted(String),
    Refused(String),
}
#[derive(Debug)]
struct Inquiry {
    uri: String,
    header: Option<String>,
}
fn read_debug_flag() -> bool {
    unsafe { DEBUG_FLAG }
}
fn verify(inq: &Inquiry) -> bool {
    if inq.uri == "/debug" && read_debug_flag() {
        return true;
    }
    inq.header.as_ref().map_or(false, |h| h == "secret")
}
fn execute(inq: Inquiry) -> Outcome {
    if verify(&inq) {
        Outcome::Granted(format!("Access granted for path: {}", inq.uri))
    } else {
        Outcome::Refused(format!("Access denied for path: {}", inq.uri))
    }
}
fn main() {
    let inquiries = vec![
        Inquiry { uri: "/debug".to_string(), header: None },
        Inquiry { uri: "/data".to_string(), header: Some("wrong".to_string()) },
        Inquiry { uri: "/data".to_string(), header: Some("secret".to_string()) },
    ];
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();
    for inq in inquiries {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let outcome = execute(inq);
            let mut guard = results.lock().unwrap();
            guard.push(outcome);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
    let guard = results.lock().unwrap();
    for outcome in guard.iter() {
        println!("{:?}", outcome);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication() {
        // The test verifies that the /debug endpoint does not incorrectly bypass authentication.
        // It creates three inquiries: one to /debug with no token, one to /data with an invalid token,
        // and one to /data with a correct token.

        // For the vulnerable implementation, the /debug inquiry would return Outcome::Granted,
        // but the secure implementation should return Outcome::Refused.
        let inquiry_debug = Inquiry { uri: "/debug".to_string(), header: None };
        let inquiry_bad = Inquiry { uri: "/data".to_string(), header: Some("wrong".to_string()) };
        let inquiry_good = Inquiry { uri: "/data".to_string(), header: Some("secret".to_string()) };

        let outcome_debug = execute(inquiry_debug);
        let outcome_bad = execute(inquiry_bad);
        let outcome_good = execute(inquiry_good);

        // The test requires that the /debug inquiry must fail authentication.
        match outcome_debug {
            Outcome::Granted(_) => {
                // This should fail for the fix; an authenticated bypass is not allowed.
                assert!(false, "Debug endpoint should not bypass authentication.");
            },
            Outcome::Refused(_) => {}
        }
        // Check that a bad token fails and a good token succeeds.
        assert!(matches!(outcome_bad, Outcome::Refused(_)),
                "Request with an incorrect token should be refused.");
        assert!(matches!(outcome_good, Outcome::Granted(_)),
                "Request with a correct token should be granted.");
    }
}
