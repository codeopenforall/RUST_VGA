use std::sync::Arc;

struct Config {
    key: String,
}

fn run_app() -> String {
    let config = Arc::new(Config {
        key: "super_secret".to_string(),
    });

    let config_clone = Arc::clone(&config);

    unsafe {
        let key_ptr = config_clone.key.as_ptr();
        let len = config_clone.key.len();
        let key_slice = std::slice::from_raw_parts(key_ptr, len);
        String::from_utf8_lossy(key_slice).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::run_app;

    #[test]
    fn encryption_test() {
        let transmitted = run_app();
        let clear_text = "super_secret";
        assert_ne!(transmitted, clear_text, "Sensitive secret is transmitted unencrypted");
    }
}