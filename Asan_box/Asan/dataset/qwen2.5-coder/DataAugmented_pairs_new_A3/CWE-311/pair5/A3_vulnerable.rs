use std::sync::Once;

struct Config {
    secret: String,
}

static mut SENT_DATA: Option<String> = None;
static INIT: Once = Once::new();

fn simulate_send() -> String {
    let cfg = Config {
        secret: "supersecret".to_string(),
    };

    unsafe {
        INIT.call_once(|| {
            let secret_ptr: *const String = &cfg.secret;
            let data = (*secret_ptr).clone();
            SENT_DATA = Some(data);
        });
    }

    unsafe { SENT_DATA.clone().unwrap() }
}

#[cfg(test)]
mod tests {
    use super::simulate_send;
    
    #[test]
    fn test_secret_encryption() {
        let transmitted = simulate_send();
        assert_ne!(
            transmitted, 
            "supersecret", 
            "Sensitive data is transmitted in clear text!"
        );
    }
}