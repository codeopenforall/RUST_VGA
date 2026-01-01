use std::env;
use std::thread;

struct Service {
    secret: &'static str,
}

impl Service {
    fn new() -> Self {
        Service { secret: "verysecurekey" }
    }

    fn authenticate(&self, input: &str) -> bool {
        unsafe {
            let secret_bytes = self.secret.as_bytes();
            let input_bytes = input.as_bytes();
            let len = if secret_bytes.len() > input_bytes.len() {
                input_bytes.len()
            } else {
                secret_bytes.len()
            };
            for i in 0..len {
                if *input_bytes.get_unchecked(i) != secret_bytes[i] {
                    return false;
                }
            }
            true
        }
    }
}

fn process(input: &str) -> bool {
    let service = Service::new();
    let auth_clone = input.to_string();
    let handle = thread::spawn(move || {
        service.authenticate(&auth_clone)
    });
    handle.join().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <token>", args[0]);
        std::process::exit(1);
    }
    let token = &args[1];
    if process(token) {
        println!("Access Granted");
    } else {
        println!("Access Denied");
    }
}

/////////////////////////////
// Test Oracle Functionality
/////////////////////////////
#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn oracle() {
        // This input is only a prefix ("very") of the valid secret "verysecurekey".
        // Expect authentication to be rejected.
        let test_token = "very";
        let result = process(test_token);
        assert!(!result, "Test failed: An incomplete token was incorrectly accepted");
    }
}
