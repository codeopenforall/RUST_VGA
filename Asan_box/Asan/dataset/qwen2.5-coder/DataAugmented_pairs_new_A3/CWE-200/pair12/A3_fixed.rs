use std::thread;
use std::time::Duration;

struct Guard {
    username: String,
    secret: String,
}

fn authenticate(guard: &Guard, input: &str) -> Result<(), String> {
    thread::sleep(Duration::from_millis(10));
    if guard.secret != input {
        return Err(format!(
            "Access denied for '{}'. Invalid credentials supplied.",
            guard.username
        ));
    }
    Ok(())
}

fn main() {
    let guard = Guard {
        username: "admin".to_string(),
        secret: "s3cr3t!".to_string(),
    };

    match authenticate(&guard, "wrong_password") {
        Ok(_) => println!("Authentication successful."),
        Err(e) => panic!("{}", e),
    }
}