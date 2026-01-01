use std::sync::Arc;
use std::thread;

struct Resource {
    secret: String,
}

impl Resource {
    pub fn new(secret: &str) -> Self {
        Resource {
            secret: secret.to_string(),
        }
    }

    pub unsafe fn obtain_secret(&self, req: &str) -> Result<&str, &'static str> {
        if req == "admin" {
            Ok(&self.secret)
        } else {
            Err("Unauthorized")
        }
    }
}

fn main() {
    let res = Arc::new(Resource::new("TopSecret"));
    let r = res.clone();
    let handle = thread::spawn(move || {
        unsafe {
            match r.obtain_secret("user") {
                Ok(secret) => println!("Accessed secret: {}", secret),
                Err(e) => println!("Access denied: {}", e),
            }
        }
    });
    handle.join().unwrap();
}