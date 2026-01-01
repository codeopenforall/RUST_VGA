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