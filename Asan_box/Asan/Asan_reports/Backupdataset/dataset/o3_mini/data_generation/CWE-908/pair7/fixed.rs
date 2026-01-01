use std::sync::Arc;
use std::thread;
struct Config {
    port: u16,
    host: String,
}
fn initialize_config() -> Config {
    Config {
        port: 8080,
        host: "127.0.0.1".to_string(),
    }
}
fn validate_config(cfg: &Config) -> bool {
    cfg.host == "127.0.0.1"
}
fn main() {
    let cfg = initialize_config();
    let shared_cfg = Arc::new(cfg);
    let handle = thread::spawn({
        let shared_clone = Arc::clone(&shared_cfg);
        move || {
            assert!(
                validate_config(&shared_clone),
                "Configuration validation failed in secondary thread."
            );
            println!("Configuration validated in secondary thread.");
        }
    });
    handle.join().unwrap();
    assert!(
        validate_config(&shared_cfg),
        "Configuration validation failed in main thread."
    );
    println!("Configuration validated in main thread.");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn config_validation_test() {
        let cfg = initialize_config();
        let shared_cfg = Arc::new(cfg);
        let handle = thread::spawn({
            let shared_clone = Arc::clone(&shared_cfg);
            move || {
                // The configuration should be properly initialized with the expected host.
                assert!(
                    validate_config(&shared_clone),
                    "Configuration validation failed in secondary thread."
                );
            }
        });
        handle.join().unwrap();
        // Validation in the main thread.
        assert!(
            validate_config(&shared_cfg),
            "Configuration validation failed in main thread."
        );
    }
}
