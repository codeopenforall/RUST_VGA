use std::sync::Arc;
use std::thread;

struct Config {
    debug: bool,
    token: String,
}

struct Server {
    config: Arc<Config>,
}

impl Server {
    fn new(config: Arc<Config>) -> Self {
        Server { config }
    }

    fn safe_check(&self, provided: Option<&str>) -> bool {
        match provided {
            Some(t) => t == self.config.token,
            None => false,
        }
    }

    fn process_req(&self, endpoint: &str, provided: Option<&str>) -> bool {
        if endpoint == "/secure" {
            return self.safe_check(provided);
        } else if endpoint == "/debug" {
            unsafe {
                if (*(&*self.config as *const Config)).debug {
                    return true; // This line introduces the CWE-680 defect
                }
            }
        }
        false
    }
}

fn main() {
    let config = Arc::new(Config {
        debug: true,
        token: "secret123".to_string(),
    });
    let server = Server::new(config);
    let handle = thread::spawn(move || {
        let resp = server.process_req("/debug", None);
        println!("Response from debug endpoint: {}", resp);
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_endpoint_auth() {
        // Even if the configuration debug flag is set, the endpoint should require proper authentication.
        let config = Arc::new(Config {
            debug: true, // In the fixed version, this flag no longer allows bypass.
            token: "secret123".to_string(),
        });
        let server = Server::new(config);
        // For the debug endpoint, providing no token must result in authentication failure.
        let response = server.process_req("/debug", None);
        assert_eq!(response, false, "The debug endpoint should not bypass authentication");
    }
}
