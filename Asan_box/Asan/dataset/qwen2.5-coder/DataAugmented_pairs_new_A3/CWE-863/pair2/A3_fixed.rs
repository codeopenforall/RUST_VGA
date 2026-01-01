pub struct Manager;

impl Manager {
    pub fn allocate_resources(role: &str, count: usize) -> Result<(), String> {
        if role != "admin" {
            return Err("Unauthorized: only admin users can allocate resources.".to_string());
        }
        const MAX_RESOURCES: usize = 50;
        if count > MAX_RESOURCES {
            return Err(format!(
                "Requested count ({}) exceeds the maximum allowed resources ({}).",
                count, MAX_RESOURCES
            ));
        }
        Ok(())
    }
}