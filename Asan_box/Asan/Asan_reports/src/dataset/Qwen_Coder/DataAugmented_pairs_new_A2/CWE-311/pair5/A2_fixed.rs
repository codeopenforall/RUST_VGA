use std::sync::Arc;
use std::thread;

struct Config {
    secret: String,
}

static mut SENT_DATA: Option<String> = None;

fn encrypt_secret(data: &str, key: u8) -> String {
    data.bytes()
        .map(|b| b ^ key)
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

fn send_data(cfg: Arc<Config>) {
    let key: u8 = 0xAA;
    let encrypted = encrypt_secret(&cfg.secret, key);
    unsafe {
        SENT_DATA = Some(encrypted);
    }
}

pub fn simulate_send() -> String {
    let cfg = Arc::new(Config { secret: "supersecret".to_string() });
    let cfg_clone = Arc::clone(&cfg);
    let handle = thread::spawn(move || {
        send_data(cfg_clone);
    });
    handle.join().unwrap();
    unsafe {
        SENT_DATA.clone().unwrap_or_default()
    }
}

fn main() {
    let transmitted = simulate_send();
    println!("Transmitted data: {}", transmitted);
}

//////////////////////////////////////////////
// Test Oracle: This function verifies that sensitive data is not transmitted in cleartext.
// It is designed to fail for the insecure version (which transmits "supersecret" directly)
// and pass for the secure version (which transmits an encrypted string).
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::simulate_send;
    
    #[test]
    fn test_secret_encryption() {
        // Triggering input includes the secret "supersecret".
        let transmitted = simulate_send();
        // The insecure version returns the cleartext "supersecret".
        // The secure version returns an encrypted string (a hex string) that must not equal "supersecret".
        assert_ne!(
            transmitted, 
            "supersecret", 
            "Sensitive data is transmitted in clear text!"
        );
    }
}
