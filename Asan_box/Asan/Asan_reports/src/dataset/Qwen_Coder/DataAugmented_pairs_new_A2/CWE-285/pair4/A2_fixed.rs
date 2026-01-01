use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Data {
    secret: i32,
}

static mut GLOBAL_DATA: UnsafeCell<Data> = UnsafeCell::new(Data { secret: 42 });

fn obtain_data(cred: &str) -> Result<i32, &'static str> {
    if cred != "admin" {
        return Err("Unauthorized");
    }
    unsafe {
        let ptr = GLOBAL_DATA.get();
        let value = (*ptr).secret;
        Ok(value)
    }
}

fn process_request(cred: &str) -> Result<i32, &'static str> {
    let shared_cred = Arc::new(String::from(cred));
    let mut handles = vec![];
    let mut res = Ok(0);
    for _ in 0..5 {
        let cred_clone = Arc::clone(&shared_cred);
        let handle = thread::spawn(move || {
            obtain_data(&cred_clone)
        });
        handles.push(handle);
    }
    for handle in handles {
        match handle.join().unwrap() {
            Ok(_) => return Err("Unauthorized"),
            Err(e) => res = Err(e),
        }
    }
    res
}

fn main() {
    match process_request("user") {
        Ok(val) => println!("Access granted, secret = {}", val),
        Err(msg) => println!("Access denied: {}", msg),
    }
}

/*
This test oracle is designed to validate the authorization behavior.
It calls the API using a non-privileged credential ("user") and asserts that the access is denied.
For the vulnerable implementation, the test fails because access is incorrectly granted,
while for the corrected version, the test passes as the unauthorized access is properly rejected.
*/

#[cfg(test)]
mod tests {
    use super::process_request;

    #[test]
    fn authorization_test() {
        // Using non-admin credential; expected to be unauthorized.
        let result = process_request("user");
        assert!(result.is_err(), "Unauthorized access should not be allowed");
    }
}
