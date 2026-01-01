use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
struct Cred {
    user: String,
    secret: String,
}
impl Cred {
    fn new(user: &str, secret: &str) -> Self {
        Cred {
            user: user.to_owned(),
            secret: secret.to_owned(),
        }
    }
}
fn process_data(data: &Arc<Mutex<Cred>>) -> String {
    let guard = data.lock().unwrap();
    let cred_copy = Cred {
        user: guard.user.clone(),
        secret: "******".to_string(),
    };
    format!("{:?}", cred_copy)
}
fn main() {
    let creds = Arc::new(Mutex::new(Cred::new("alice", "s3cr3t")));
    let creds_clone = Arc::clone(&creds);
    let handle = thread::spawn(move || {
        process_data(&creds_clone)
    });
    let output = handle.join().unwrap();
    println!("{}", output);
}