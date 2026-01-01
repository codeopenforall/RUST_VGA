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

#[cfg(test)]
mod tests {
    use super::Manager;

    // Test oracle: For a non-admin role, the call should return an error.
    #[test]
    fn test_oracle() {
        // Expect an error for unauthorized user in the improved code.
        let result = Manager::allocate_resources("user", 5);
        assert!(result.is_err(), "Unauthorized role should result in an error.");
    }
}
