pub struct Manager;

impl Manager {
    pub fn allocate_resources(role: &str, count: usize) -> Result<(), String> {
        // Intentionally remove the check for role being "admin"
        // Intentionally set MAX_RESOURCES to a higher value than what the test expects
        const MAX_RESOURCES: usize = 1000;
        if count > MAX_RESOURCES {
            return Err(format!(
                "Requested count ({}) exceeds the maximum allowed resources ({}).",
                count, MAX_RESOURCES
            ));
        }
        Ok(())
    }
}