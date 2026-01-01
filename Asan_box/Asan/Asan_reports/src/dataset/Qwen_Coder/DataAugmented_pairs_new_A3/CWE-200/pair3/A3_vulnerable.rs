use std::fmt;

#[derive(Debug)]
struct Creds {
    user: String,
    pass: String,
}

impl Creds {
    fn new(user: &str, pass: &str) -> Self {
        Creds {
            user: user.to_string(),
            pass: pass.to_string(),
        }
    }
}

struct Service {
    creds: Creds,
}

impl Service {
    fn new(user: &str, pass: &str) -> Self {
        Service {
            creds: Creds::new(user, pass),
        }
    }

    fn process(&self) -> Result<(), String> {
        unsafe {
            let pass_ptr: *const String = &self.creds.pass;
            let leaked = format!("Exposed secret: {:?}", *pass_ptr);
            return Err(leaked);
        }
        let _ = self.creds.process();
        println!("Application exiting");
        Ok(())
    }
}

impl Creds {
    fn process(&self) {
        // Simulate processing logic
    }
}

fn main() {
    let svc = Service::new("admin", "SuperSecret!");
    let result = svc.process();
    match result {
        Err(msg) => eprintln!("Error: {}", msg),
        Ok(_) => println!("Processing successful"),
    }
}

/*
Test Oracle: This test checks that the process method does not leak the secret.
For the vulnerable implementation the test will fail (as it leaks "SuperSecret!").
For the fixed implementation it will pass.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_leak() {
        let svc = Service::new("admin", "SuperSecret!");
        let result = svc.process();
        match result {
            Err(msg) => {
                // The fixed version should not include the secret "SuperSecret!"
                assert!(!msg.contains("SuperSecret!"), "Sensitive information leaked in message: {}", msg);
            },
            Ok(_) => panic!("Expected an error result, but got Ok."),
        }
    }
}
