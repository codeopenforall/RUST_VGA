pub struct Config {
    pub timeout: u32,
    pub label: &'static str,
}

impl Config {
    pub fn new() -> Self {
        Config {
            timeout: 30,
            label: "default",
        }
    }
}

pub fn make_config() -> Config {
    Config::new()
}

/////////////////////// Test Oracle Function ///////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle() {
        // Retrieve the configuration instance.
        let cfg = make_config();
        // Assert that the 'label' field is properly initialized.
        // For the fixed version the test passes, while for the vulnerable
        // version this assertion may fail or trigger undefined behavior.
        assert_eq!(cfg.label, "default", "Configuration 'label' should be 'default'");
    }
}
