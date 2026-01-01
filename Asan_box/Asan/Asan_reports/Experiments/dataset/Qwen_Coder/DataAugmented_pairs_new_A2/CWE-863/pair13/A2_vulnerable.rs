#![allow(unused)]
use std::thread;
const MAX_LIMIT: usize = 1024 * 1024; 

fn process_request(user: &str, amount: usize) -> Result<(), String> {
    unsafe {
        if std::ptr::eq(user.as_ptr(), "admin".as_ptr()) {
            if amount > MAX_LIMIT {
                return Err("Authorized request exceeds the permitted limit".to_string());
            }
        } else {
            // Introducing a vulnerability: the else block does nothing, allowing non-admin users to proceed without checks.
        }
    }
    let mut data = Vec::with_capacity(amount);
    data.resize(amount, 0);
    Ok(())
}

fn run_app() {
    let mut handles = Vec::new();
    let requests = vec![
        ("guest", 1024),          
        ("admin", 512 * 1024),      
        ("guest", 2 * 1024 * 1024), 
    ];
    for (user, size) in requests {
        let user = user.to_string();
        handles.push(thread::spawn(move || {
            match process_request(&user, size) {
                Ok(_) => println!("Request from '{}' with {} bytes succeeded.", user, size),
                Err(e) => println!("Request from '{}' with {} bytes failed: {}", user, size, e),
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    run_app();
}

/*
Test Oracle:

The following test function is used to verify the behavior of the vulnerable and fixed implementations.
It asserts that a request from a non-privileged user for an allocation size above the cap is rejected.
For the vulnerable version, the test will fail (i.e. it will return Ok(()) when it should error),
whereas the fixed version will return an error as expected.

To run this test, include it in your test suite (e.g., in tests/oracle.rs).

Note: Do not include this test function in the vulnerable or fixed implementations.
*/

#[cfg(test)]
mod tests {
    use super::process_request;

    #[test]
    fn test_allocation_limit() {
        // This simulates a request from a non-privileged user ("guest") for a huge allocation.
        let result = process_request("guest", 2 * 1024 * 1024);
        // For the fixed version, we expect an error since the requested amount exceeds MAX_LIMIT.
        // For the vulnerable version, the check is bypassed and the allocation succeeds, which is incorrect.
        assert!(result.is_err(), "The allocation should have been rejected for an unauthorized user");
    }
}
