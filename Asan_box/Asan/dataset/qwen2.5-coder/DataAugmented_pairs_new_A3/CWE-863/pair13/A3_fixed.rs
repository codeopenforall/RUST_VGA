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