use std::fmt;
use std::thread;

#[derive(Debug)]
struct Config {
    name: String,
    api_key: String,
}

impl Config {
    fn new() -> Self {
        Config {
            name: "ServerAlice".into(),
            api_key: "SECRET123".into(),
        }
    }
}

fn mask_api_key(config: &Config) -> String {
    format!("Config {{ name: {:?}, api_key: \"{}\" }}", config.name, "*".repeat(config.api_key.len()))
}

fn run_session() -> String {
    let conf = Config::new();
    let debug_str = mask_api_key(&conf);
    let log_output = unsafe {
        let ptr = debug_str.as_ptr();
        let len = debug_str.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf8_lossy(slice).to_string()
    };
    let handle = thread::spawn(move || {
        mask_api_key(&conf)
    });
    let thread_log = handle.join().unwrap();
    format!("{}; {}", log_output, thread_log)
}

fn main() {
    let output = run_session();
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::run_session;

    #[test]
    fn test_sensitive_data_not_leaked() {
        let output = run_session();
        // The fixed code masks the secret API key.
        // For the vulnerable version, this assertion would fail because "SECRET123" appears in the log.
        assert!(
            !output.contains("SECRET123"),
            "Sensitive information leaked in logs!"
        );
        // Additionally, ensure that masked text is present.
        assert!(
            output.contains("********"),
            "Masked secret was not found in logs!"
        );
    }
}
