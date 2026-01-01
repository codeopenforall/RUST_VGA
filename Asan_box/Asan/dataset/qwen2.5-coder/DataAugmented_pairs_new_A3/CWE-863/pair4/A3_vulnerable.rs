struct ResourceService;

impl ResourceService {
    pub fn execute_request(&self, num: usize) -> Result<(), String> {
        // Intentionally omitting the check for the safe limit
        // This will cause the test to fail as the request will not be rejected
        Ok(())
    }
}

fn main() {
    let service = ResourceService;
    if let Err(e) = service.execute_request(1000) {
        eprintln!("Error: {}", e);
    } else {
        println!("Request processed successfully.");
    }
}