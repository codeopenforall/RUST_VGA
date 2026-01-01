use std::env;

pub struct Runner;

impl Runner {
    pub fn execute(&self) -> Result<String, String> {
        let user_input = unsafe {
            let ptr = env::var("SEARCH_PATH").ok().map(|s| {
                let boxed = Box::new(s);
                Box::into_raw(boxed)
            });
            let result = match ptr {
                Some(raw) => {
                    let string_ref = &*raw;
                    string_ref.clone()
                }
                None => String::new(),
            };
            result
        };

        env::set_var("PATH", user_input.clone());

        Ok(user_input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_oracle() {
        // Set a malicious environment variable that is not an absolute path.
        env::set_var("SEARCH_PATH", "malicious");

        // Create a runner instance and execute the function.
        let runner = Runner;
        let result = runner.execute().expect("Execution failed");

        // For the secure version, the unsanitized input "malicious" should be rejected,
        // and the safe default PATH "/usr/bin:/bin" should be used.
        assert_eq!(
            result, "/usr/bin:/bin",
            "The environment PATH was not sanitized properly"
        );
    }
}
