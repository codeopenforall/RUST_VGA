const MAX_LIMIT: usize = 1024 * 1024;

pub fn process_request(user: &str, amount: usize) -> Result<(), String> {
    if user != "admin" && amount > MAX_LIMIT {
        return Err("Unauthorized request exceeds the permitted limit".to_string());
    }
    Ok(())
}

fn main() {
    // Example usage
    match process_request("admin", 2 * 1024 * 1024) {
        Ok(_) => println!("Request processed successfully"),
        Err(e) => println!("Error: {}", e),
    }

    match process_request("guest", 2 * 1024 * 1024) {
        Ok(_) => println!("Request processed successfully"),
        Err(e) => println!("Error: {}", e),
    }
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
